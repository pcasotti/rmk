mod behavior;
mod bind_interrupt;
mod ble;
mod chip_init;
mod comm;
mod controller;
mod entry;
mod feature;
mod flash;
mod gpio_config;
mod import;
mod input_device;
mod keyboard;
mod keyboard_config;
mod layout;
mod matrix;
mod split;

use darling::FromMeta;
use darling::ast::NestedMeta;
use proc_macro::TokenStream;
use quote::quote;
use split::peripheral::parse_split_peripheral_mod;
use syn::parse_macro_input;

use crate::keyboard::parse_keyboard_mod;

#[proc_macro_attribute]
pub fn rmk_keyboard(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_mod = parse_macro_input!(item as syn::ItemMod);
    parse_keyboard_mod(item_mod).into()
}

#[proc_macro_attribute]
pub fn rmk_central(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_mod = parse_macro_input!(item as syn::ItemMod);
    parse_keyboard_mod(item_mod).into()
}

/// Attribute for `rmk_peripheral` macro
#[derive(Debug, FromMeta)]
struct PeripheralAttr {
    #[darling(default)]
    id: usize,
}

#[proc_macro_attribute]
pub fn rmk_peripheral(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_mod = parse_macro_input!(item as syn::ItemMod);
    let attr_args = match NestedMeta::parse_meta_list(attr.clone().into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(darling::Error::from(e).write_errors());
        }
    };

    let peripheral_id = match PeripheralAttr::from_list(&attr_args) {
        Ok(v) => v.id,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    parse_split_peripheral_mod(peripheral_id, attr, item_mod).into()
}

struct DispatchMapping {
    endpoint: syn::Type,
    _eq: syn::Token![=],
    handler: syn::Ident,
}

impl syn::parse::Parse for DispatchMapping {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(DispatchMapping {
            endpoint: input.parse()?,
            _eq: input.parse()?,
            handler: input.parse()?,
        })
    }
}

#[proc_macro_attribute]
pub fn dispatcher(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mappings = parse_macro_input!(attr with syn::punctuated::Punctuated::<DispatchMapping, syn::Token![;]>::parse_terminated);

    let struct_item = parse_macro_input!(item as syn::ItemStruct);
    let struct_name = &struct_item.ident;
    let (impl_generics, ty_generics, where_clause) = struct_item.generics.split_for_impl();

    let match_arms = mappings.iter().map(|mapping| {
        let endpoint_type = &mapping.endpoint;
        let handler_ident = &mapping.handler;

        quote! {
            <#endpoint_type as ::rmk_types::protocol::rmk_rpc::Endpoint>::KEY => {
                let request = ::postcard::from_bytes::<<#endpoint_type as ::rmk_types::protocol::rmk_rpc::Endpoint>::Request>(&data[1..])?;
                let response = self.#handler_ident(request).await;
                ::postcard::to_slice(&response, &mut buf[1..])?;
            }
        }
    });

    let expanded = quote! {
        #struct_item

        impl #impl_generics #struct_name #ty_generics #where_clause {
            async fn handle(&mut self, data: &[u8; 32]) -> ::postcard::Result<[u8; 32]> {
                let mut buf = [0; 32];
                let key = data[0];
                buf[0] = key;
                match key {
                    #(#match_arms)*
                    _ => {
                        info!("Unknown cmd: {:?}", data);
                        return Err(::postcard::Error::DeserializeBadEncoding);
                    }
                }
                Ok(buf)
            }
        }
    };

    expanded.into()
}
