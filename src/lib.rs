use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) {
    let message = format!("Hi there {}", name);
    alert(&message);
}

#[wasm_bindgen]
extern {
    pub fn alert(message: &str);
}
