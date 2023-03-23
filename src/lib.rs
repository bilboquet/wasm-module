// `wai_bindgen_rust` tells we are doing the work in Rust
// `export` tells that we provide
wai_bindgen_rust::export!("wasm-mod-exported.wai");

// what we are using from the outside
wai_bindgen_rust::import!("host-exported.wai");


// looks like wai_bindgen_rust::export is buggy on this point it forget to declare this struct
// it's documented in the example
struct WasmModExported;

// implement in this impl block what is exported in the .wai file
impl wasm_mod_exported::WasmModExported for WasmModExported {
    fn add(a: u32, b: u32) -> u32 {
        a + b
    }

    fn start() -> u32 {
        host_exported::echo();
        0u32
    }
}
