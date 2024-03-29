const { startup, add_two_numbers } = wasm_bindgen

async function run_wasm() {
  // Load the wasm file by awaiting the Promise returned by `wasm_bindgen`
  // `wasm_bindgen` was imported in `index.html`
  await wasm_bindgen('./pkg/wasm_no_modules_wasm_worker_bg.wasm')

  console.log('index.js loaded')

  // Run main Wasm entry point which will create a worker
  startup()

  // Demonstrate that we can call our function imported from wasm
  console.log('2 + 4 = ', add_two_numbers(2, 4))
}

run_wasm()
