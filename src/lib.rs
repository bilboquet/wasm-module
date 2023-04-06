// `wai_bindgen_rust` tells we are doing the work in Rust
// `export` tells that we provide
wai_bindgen_rust::export!("wasm-mod-exported.wai");

// what we are using from the outside
wai_bindgen_rust::import!("host-exported.wai");

// equivalent to `wasmer run wasmer/wai-bindgen-cli --dir=. -- rust-wasm --import env.wai`
wai_bindgen_rust::import!("env.wai");

// looks like wai_bindgen_rust::export is buggy on this point it forget to declare this struct
// it's documented in the example
struct WasmModExported;

// implement in this impl block what is exported in the .wai file
impl wasm_mod_exported::WasmModExported for WasmModExported {
    fn add(a: u32, b: u32) -> u32 {
        a + b
    }

    fn call_hello() -> i32 {
        host_exported::hello();
        0i32
    }

    fn call_zero_arg() -> i32 {
        env::zero_arg();
        0i32
    }

    fn call_one_arg() -> i32 {
        env::one_arg(1);
        0i32
    }

    fn call_two_args() -> i32 {
        env::two_args(2, 1);
        0i32
    }

    fn call_one_string_arg() -> i32 {
        env::one_string_arg("one string arg");
        0i32
    }

    fn call_one_array_arg() -> i32 {
        env::one_array_arg(&[1, 2, 3]);
        0i32
    }

    fn call_abort() -> i32 {
        env::abort("message", 0, 0);
        0i32
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
