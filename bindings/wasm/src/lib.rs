// build the json, handle Ok, and error 
// handle macros
use regxact_core::Rx;

#[wasm_bindgen]
pub fn create(pattern: &str) -> String {
    to_json(Rx::create(pattern))
}

fn to_json(result: Result<Rx, RegxactError>) -> String{
    match result {
        // Ok(Rx)=>serde_json::to_string()
        
    }
}
