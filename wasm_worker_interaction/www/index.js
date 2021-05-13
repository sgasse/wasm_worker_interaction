const {startup} = wasm_bindgen;

async function run_wasm() {
    // Load the wasm file by awaiting the Promise returned by `init()` to resolve
    await wasm_bindgen('./pkg/wasm_worker_interaction_bg.wasm');

    console.log('index.js loaded');

    // Run main WASM entry point which will create a worker
    startup();
}

run_wasm();
