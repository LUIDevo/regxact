use wasm_bindgen::prelude::*;
use crate::builder::RegxactBuilder;
use crate::rx::Rx;

#[wasm_bindgen]
pub struct WasmRegxact {inner: Rx}

#[wasm_bindgen]
impl WasmRegxact {

    #[wasm_bindgen(constructor)]
    pub fn new(pattern: &str) ->
        Result<WasmRegxact, JsError> {
            RegxactBuilder::new(pattern).build()
                .map(|inner| WasmRegxact { inner })
                .map_err(|e|
                    JsError::new(&format!("{e:?}")))
    }

    pub fn allow(mut self, flag: &str) ->
        Result<WasmRegxact, JsError> {
            self.inner =
                self.inner.allow(flag).map_err(|e|
                    JsError::new(&format!("{e:?}")))?;
            Ok(self)
    }

    #[wasm_bindgen(getter)]
    pub fn pattern(&self) -> String {
        self.inner.pattern.clone() 
    }
}
