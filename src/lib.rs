use wasm_bindgen::prelude::*;

// extern tells Rust that we want to call some externally defined functions. The attribute says "wasm-bindgen knows how to find these functions".
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

// function for javascript to call
// (attribute is modifying this fn too)
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {name}!"));
}
