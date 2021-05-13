import init, {add_two_numbers} from './pkg/wasm_main_js_worker.js';

async function run_wasm() {
    // Load the wasm file by awaiting the Promise returned by `init()` to resolve
    await init();

    console.log('index.js loaded');

    // Demonstrate that we can call our function imported from wasm
    console.log('2 + 4 = ', add_two_numbers(2, 4));

    // Create a worker in JS - note the `{type: 'module'}`
    var myWorker = new Worker('./worker.js', {type: 'module'});
}

run_wasm();
