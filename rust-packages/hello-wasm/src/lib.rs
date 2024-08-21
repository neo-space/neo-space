
use wasm_bindgen::prelude::*;

// this macro generates the necessary boilerplate to allow
// the rust function to be called from boilerplate
#[wasm_bindgen]
extern "C" {
    // this allows Rust to call the Javascript alert function
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}