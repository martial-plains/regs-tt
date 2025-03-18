fn main() {
    println!("cargo:rustc-cfg=getrandom_backend=\"wasm_js\"");
}
