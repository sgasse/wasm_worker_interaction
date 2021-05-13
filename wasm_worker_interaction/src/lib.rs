use std::cell::RefCell;
use std::rc::Rc;
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


    let worker_handle = Rc::new(RefCell::new(Worker::new("./worker.js").unwrap()));
    console::log_1(&"Created a new worker from within WASM".into());

    setup_input_onchange_callback(worker_handle.clone());
}

fn setup_input_onchange_callback(worker: Rc<RefCell<web_sys::Worker>>) {
    let document = web_sys::window().unwrap().document().unwrap();

    let callback = Closure::wrap(Box::new(move || {
        let document = web_sys::window().unwrap().document().unwrap();

        let input_field = document.get_element_by_id("inputNumber")
            .expect("#inputNumber should exist");
        let input_field = input_field.dyn_ref::<HtmlInputElement>()
            .expect("#inputNumber should be a HtmlInputElement");

        let number = match input_field.value().parse::<i32>() {
            Ok(num) => num,
            Err(_) => 0,
        };

        {
            let worker_handle = &*worker.borrow();
            let _ = worker_handle.post_message(&number.into());
        }

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

#[wasm_bindgen]
pub fn log_double(number: i32) {
    let double_number = 2*number;
    console::log_2(&"Double input number: ".into(), &double_number.into());
}

#[wasm_bindgen]
pub struct NumberEval {
    number: i32,
}

#[wasm_bindgen]
impl NumberEval {
    pub fn new(init_number: i32) -> NumberEval {
        NumberEval{number: init_number}
    }

    pub fn get_last_number(&self) -> i32 {
        self.number
    }

    pub fn is_even(&mut self, number: i32) -> bool {
        self.number = number;
        match self.number % 2 {
            0 => true,
            _ => false,
        }
    }
}

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}