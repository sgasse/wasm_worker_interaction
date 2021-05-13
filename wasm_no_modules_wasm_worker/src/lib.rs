use wasm_bindgen::prelude::*;
use web_sys::{console, Worker};

#[wasm_bindgen]
pub fn add_two_numbers(a: i32, b: i32) -> i32 {
    return a + b;
}

#[wasm_bindgen]
pub fn startup() {
    // Until deploying a WASM application in production, adding the panic hook
    // makes debugging a lot easier.
    set_panic_hook();

    Worker::new("./worker.js");
    console::log_1(&"Created a new worker from within WASM".into());

}

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}