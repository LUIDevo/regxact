use wasm_bindgen::prelude::*;
use crate::builder::RegxactBuilder;
use crate::rx::Rx;

#[wasm_bindgen]
pub struct WasmRegxact {inner: Rx}
