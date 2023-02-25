const {add_two_numbers} = wasm_bindgen;

async function run_wasm() {
    // Load the wasm file by awaiting the Promise returned by `wasm_bindgen`
    // `wasm_bindgen` was imported in `index.html`
    await wasm_bindgen('./pkg/wasm_no_modules_js_worker_bg.wasm');

    console.log('index.js loaded');

    // Demonstrate that we can call our function imported from wasm
    console.log('2 + 4 = ', add_two_numbers(2, 4));

    // Create a worker in JS - no option `{type: 'module'}`
    var myWorker = new Worker('./worker.js');
}

run_wasm();
