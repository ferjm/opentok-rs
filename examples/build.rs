use std::env;

fn main() {
    if let Ok(sdk_dir) = env::var("OPENTOK_PATH") {
        let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        println!(
            "cargo:rustc-link-search=native={}/../{}/lib",
            root_dir, sdk_dir
        );
    }
    println!("cargo:rerun-if-changed=build.rs");
}
