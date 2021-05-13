importScripts('./pkg/wasm_no_modules_js_worker.js');

const {add_two_numbers} = wasm_bindgen;

// We compiled with `--target no-modules`, which does not create a module. The generated bindings
// can be loaded in web workers in all modern browsers.
console.log('Hello from worker')

async function run_in_worker() {
    // Load the wasm file by awaiting the Promise returned by `wasm_bindgen`.
    await wasm_bindgen('./pkg/wasm_no_modules_js_worker_bg.wasm');

    console.log('3 + 5 = ', add_two_numbers(3, 5));
}

run_in_worker();