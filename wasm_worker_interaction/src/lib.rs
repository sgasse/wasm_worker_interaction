use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, HtmlInputElement, Worker};

#[wasm_bindgen]
pub fn add_two_numbers(a: i32, b: i32) -> i32 {
    return a + b;
}

#[wasm_bindgen]
pub fn startup() {
    // Until deploying a WASM application in production, adding the panic hook
    // makes debugging a lot easier.
    set_panic_hook();

    setup_input_onchange_callback();

    Worker::new("./worker.js");
    console::log_1(&"Created a new worker from within WASM".into());
}

fn setup_input_onchange_callback() {
    let document = web_sys::window().unwrap().document().unwrap();

    let callback = Closure::wrap(Box::new(move || {
        console::log_1(&"Onchange callback".into());
    }) as Box<dyn FnMut()>);

    document
        .get_element_by_id("inputNumber")
        .expect("#inputNumber should exist")
        .dyn_ref::<HtmlInputElement>()
        .expect("#inputNumber should be a HtmlInputElement")
        .set_oninput(Some(callback.as_ref().unchecked_ref()));

    // Leaks memory
    callback.forget();
}

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}