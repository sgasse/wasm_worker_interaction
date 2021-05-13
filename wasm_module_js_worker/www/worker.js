import init, {add_two_numbers} from './pkg/wasm_main_js_worker.js';

// We compiled with `--target web`, which creates an ES module. Not all modern browsers have support
// for loading modules in web workers. This example will work in Chrome but not in Firefox.
console.log('Hello from worker')

async function run_in_worker() {
    // Loading wasm file
    await init();

    console.log('3 + 5 = ', add_two_numbers(3, 5));
}

run_in_worker();