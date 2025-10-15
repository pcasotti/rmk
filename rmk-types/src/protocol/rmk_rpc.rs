use postcard::experimental::max_size::MaxSize;
use postcard_schema::Schema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::action::KeyAction;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Schema)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct KeyActionPos {
    pub col: u8,
    pub row: u8,
    pub layer: u8,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Schema)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KeyActionError {
    InvalidPos,
    InvalidLayer,
}

pub trait Endpoint {
    type Request: Schema + Serialize + DeserializeOwned;
    type Response: Schema + Serialize + DeserializeOwned;
    const REQ_KEY: [u8; 8];
    const RESP_KEY: [u8; 8];
}

macro_rules! endpoint {
    ($tyname:ident, $req:ty, $resp:ty) => {
        pub struct $tyname;

        impl Endpoint for $tyname {
            type Request = $req;
            type Response = $resp;
            const REQ_KEY: [u8; 8] = postcard_rpc::hash::fnv1a64::hash_ty_path::<$req>(stringify!($tyname));
            const RESP_KEY: [u8; 8] = postcard_rpc::hash::fnv1a64::hash_ty_path::<$resp>(stringify!($tyname));
        }
    };
}

endpoint!(GetActiveLayer, (),                        u8);
endpoint!(GetKeyAction,   KeyActionPos,              Result<KeyAction, KeyActionError>);
endpoint!(SetKeyAction,   (KeyActionPos, KeyAction), Result<(), KeyActionError>);

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Schema, MaxSize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Header {
    pub key: [u8; 8],
    pub seq_no: u16,
}

impl Header {
    pub const SIZE: usize = Self::POSTCARD_MAX_SIZE;
}
