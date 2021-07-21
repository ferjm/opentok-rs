extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=native=./libopentok/lib");
    println!("cargo:rustc-link-lib=opentok");

    println!("cargo:rerun-if-changed=./libopentok/include/opentok.h");

    let bindings = bindgen::Builder::default()
        .header("./libopentok/include/opentok.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_var("OTC_.*")
        .allowlist_type("otc_.*")
        .allowlist_function("otc_.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
