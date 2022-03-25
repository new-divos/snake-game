use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet(name: &str) {
    let message = format!("Hi there {}", name);
    alert(&message);
}

#[wasm_bindgen]
extern {
    pub fn alert(message: &str);
}
