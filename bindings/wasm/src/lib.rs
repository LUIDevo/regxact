// build the json, handle Ok, and error 
// handle macros


#[wasm_bindgen]
pub fn create(pattern: &str) -> String {
    to_json(Rx::create(pattern))
}
