use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add_two_numbers(a: i32, b: i32) -> i32 {
    return a + b;
}

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}