# Wasm experiment

## Note on client (aka wasm) side

### 1

follow https://wasmerio.github.io/wasmer-pack/user-docs/tutorial/01-hello-world.html

### 2

It looks like `wai_bindgen_rust::export` is buggy on a point it forget to declare the struct that name the trait.
So simply declare it like:

```Rust
wai_bindgen_rust::export!("hello.wai");
// add v-- according to ---^
struct Hello;

```

to force the target to wasm add this file to the Rust project

```
File: .cargo/config.toml
────────────────────────
[build]
target = "wasm32-unknown-unknown"
```

Experimental Rust is required, hence the `rust-toolchain.toml`

/!\\

if used `cargo install wasmer-pack-cli` has to be executed in the context of the `rust-toolchain.toml` above, else strange link errors will happen.

### 3

Compared to the tutorial `[package.metadata.wapm]` and `[package.metadata.wapm.bindings]` in Cargo.toml changed a bit. See the file.

One can use `cargo wapm --dry-run --out-dir <DIR>` to simply export locally the wasm module and its `.wai` file.