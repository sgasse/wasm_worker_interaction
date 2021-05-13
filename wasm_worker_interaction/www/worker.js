importScripts('./pkg/wasm_worker_interaction.js');

console.log('Hello from worker')

const {NumberEval} = wasm_bindgen;

async function init_wasm_in_worker() {
    await wasm_bindgen('./pkg/wasm_worker_interaction_bg.wasm');

    var num_eval = NumberEval.new(0);

    self.onmessage = async event => {
        console.log("Last number: ", num_eval.get_last_number());
        if (num_eval.is_even(event.data)) {
            console.log("Current number is even");
        } else {
            console.log("Current number is odd");
        }
    };
};

init_wasm_in_worker();
