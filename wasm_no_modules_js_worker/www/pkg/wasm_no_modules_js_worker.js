let wasm_bindgen;
(function() {
    const __exports = {};
    let script_src;
    if (typeof document === 'undefined') {
        script_src = location.href;
    } else {
        script_src = new URL(document.currentScript.src, location.href).toString();
    }
    let wasm;
    /**
    * @param {number} a
    * @param {number} b
    * @returns {number}
    */
    __exports.add_two_numbers = function(a, b) {
        const ret = wasm.add_two_numbers(a, b);
        return ret;
    };

    async function load(module, imports) {
        if (typeof Response === 'function' && module instanceof Response) {
            if (typeof WebAssembly.instantiateStreaming === 'function') {
                try {
                    return await WebAssembly.instantiateStreaming(module, imports);

                } catch (e) {
                    if (module.headers.get('Content-Type') != 'application/wasm') {
                        console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                    } else {
                        throw e;
                    }
                }
            }

            const bytes = await module.arrayBuffer();
            return await WebAssembly.instantiate(bytes, imports);

        } else {
            const instance = await WebAssembly.instantiate(module, imports);

            if (instance instanceof WebAssembly.Instance) {
                return { instance, module };

            } else {
                return instance;
            }
        }
    }

    function getImports() {
        const imports = {};
        imports.wbg = {};

        return imports;
    }

    function initMemory(imports, maybe_memory) {

    }

    function finalizeInit(instance, module) {
        wasm = instance.exports;
        init.__wbindgen_wasm_module = module;


        return wasm;
    }

    function initSync(module) {
        const imports = getImports();

        initMemory(imports);

        if (!(module instanceof WebAssembly.Module)) {
            module = new WebAssembly.Module(module);
        }

        const instance = new WebAssembly.Instance(module, imports);

        return finalizeInit(instance, module);
    }

    async function init(input) {
        if (typeof input === 'undefined') {
            input = script_src.replace(/\.js$/, '_bg.wasm');
        }
        const imports = getImports();

        if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
            input = fetch(input);
        }

        initMemory(imports);

        const { instance, module } = await load(await input, imports);

        return finalizeInit(instance, module);
    }

    wasm_bindgen = Object.assign(init, { initSync }, __exports);

})();
