importScripts('./pkg/wasm_worker_interaction.js');

console.log('Initializing worker')

const {NumberEval} = wasm_bindgen;

async function init_wasm_in_worker() {
    await wasm_bindgen('./pkg/wasm_worker_interaction_bg.wasm');

    var num_eval = NumberEval.new(0);

    self.onmessage = async event => {
        var worker_result = num_eval.is_even(event.data);
        self.postMessage(worker_result);
    };
};

init_wasm_in_worker();
