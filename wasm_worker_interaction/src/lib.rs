use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, HtmlInputElement, MessageEvent, Worker};

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
#[wasm_bindgen]
pub fn startup() {
    // Until deploying a WASM application in production, adding the panic hook
    // makes debugging a lot easier.
    set_panic_hook();


    let worker_handle = Rc::new(RefCell::new(Worker::new("./worker.js").unwrap()));
    console::log_1(&"Created a new worker from within WASM".into());

    setup_input_onchange_callback(worker_handle.clone());
}

fn get_on_msg_callback() -> Closure<dyn FnMut(MessageEvent)> {
    let callback = Closure::wrap(Box::new(move |event: MessageEvent | {
        console::log_2(&"Received response: ".into(), &event.data().into());
    }) as Box<dyn FnMut(_)>);

    callback
}

fn setup_input_onchange_callback(worker: Rc<RefCell<web_sys::Worker>>) {
    let document = web_sys::window().unwrap().document().unwrap();

    // If our `onmessage` callback should stay valid after exiting from the `onchange` closure,
    // we need to either forget it (so it is not destroyed) or store it somewhere.
    // To avoid leaking memory every time we want to receive a response from the worker, we
    // move a handle into the `onchange` closure to which we will always attach the last `onmessage`
    // callback. The initial value will not be used and we silence the warning.
    #[allow(unused_assignments)]
    let mut persistent_callback_handle = get_on_msg_callback();

    let callback = Closure::wrap(Box::new(move || {
        console::log_1(&"onchange callback triggered".into());
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
            persistent_callback_handle = get_on_msg_callback();
            worker_handle.set_onmessage(Some(persistent_callback_handle.as_ref().unchecked_ref()));

        }

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