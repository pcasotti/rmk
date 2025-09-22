use postcard_rpc::endpoint;
use postcard_schema::Schema;
use serde::{Deserialize, Serialize};

use crate::action::KeyAction;

#[derive(Debug, Serialize, Deserialize, Schema)]
pub struct KeyActionPos {
    pub col: u8,
    pub row: u8,
    pub layer: u8,
}

#[derive(Debug, Serialize, Deserialize, Schema)]
pub enum KeyActionError {
    InvalidPos,
    InvalidLayer,
}

pub trait Endpoint {
    type Request;
    type Response;
    const KEY: u8;
}

pub struct GetKeyAction;
impl Endpoint for GetKeyAction {
    type Request = KeyActionPos;
    type Response = Result<KeyAction, KeyActionError>;
    const KEY: u8 = 0x00;
}

pub struct SetKeyAction;
impl Endpoint for SetKeyAction {
    type Request = (KeyActionPos, KeyAction);
    type Response = Result<(), KeyActionError>;
    const KEY: u8 = 0x01;
}

pub struct GetActiveLayer;
impl Endpoint for GetActiveLayer {
    type Request = ();
    type Response = u8;
    const KEY: u8 = 0x02;
}

// endpoint!(GetKeyAction, KeyActionPos, Result<KeyAction, KeyActionError>);
// endpoint!(SetKeyAction, (KeyActionPos, KeyAction), Result<(), KeyActionError>);
