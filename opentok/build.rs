use std::env;

fn main() {
    if let Ok(sdk_dir) = env::var("OPENTOK_PATH") {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let lib_path = format!("{}/../{}/lib", manifest_dir, sdk_dir);
        println!("cargo:rustc-link-search=native={}", lib_path);
        println!("cargo:rustc-env=LD_LIBRARY_PATH={}", lib_path);
    }
}
