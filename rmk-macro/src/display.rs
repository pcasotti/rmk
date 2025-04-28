//! Initialize light config boilerplate of RMK, including USB or BLE
//!
use quote::quote;

use crate::config::PinConfig;
use crate::gpio_config::convert_gpio_str_to_output_pin;
use crate::keyboard_config::KeyboardConfig;
use crate::ChipModel;

pub(crate) fn build_display_config(chip: &ChipModel, pin_config: &Option<PinConfig>) -> proc_macro2::TokenStream {
    match pin_config {
        Some(c) => {
            let p = convert_gpio_str_to_output_pin(chip, c.pin.clone(), c.low_active);
            quote! { Some(#p) }
        }
        None => quote! {None},
    }
}

pub(crate) fn expand_display_config(keyboard_config: &KeyboardConfig) -> proc_macro2::TokenStream {
    let scl = build_display_config(&keyboard_config.chip, &keyboard_config.display.scl);
    let sda = build_display_config(&keyboard_config.chip, &keyboard_config.display.sda);

    // Generate a macro that does light config
    quote! {
        let display_config = ::rmk::config::DisplayConfig {
            scl: #scl,
            sda: #sda,
        };
    }
}
