// `wai_bindgen_rust` tells we are doing the work in Rust
// `export` tells that we provide
wai_bindgen_rust::export!("wasm-mod-exported.wai");

// what we are using from the outside
wai_bindgen_rust::import!("host-exported.wai");
wai_bindgen_rust::import!("env.wai");

// looks like wai_bindgen_rust::export is buggy on this point it forget to declare this struct
// it's documented in the example
struct WasmModExported;

// implement in this impl block what is exported in the .wai file
impl wasm_mod_exported::WasmModExported for WasmModExported {
    fn add(a: u32, b: u32) -> u32 {
        a + b
    }

    fn start() -> u32 {
        host_exported::hello();
        env::zero_arg();
        env::one_arg(1);
        env::two_arg(2, 1);
        env::one_string_arg("one string arg");
        env::abort("message", 0, 0);
        0u32
    }

    fn fillarray_u8(len: u32, value: u8) -> Vec<u8> {
        vec![value; len as usize]
    }

    fn fillarray_static_u8(args: Vec<u8>) -> Vec<u8> {
        args.iter().map(|x| x + 1).collect()
    }

    // take ownership of args because wasm run in sandbox
    fn greet(name: String) -> String {
        format!("Hello {}!", name)
    }
}
