extern crate bindgen;

use std::env;
use std::path::PathBuf;

// Clang extra args env variable name
const MOSQUITTO_PLUGIN_CLANG_EXTRA_ARGS: &str = "MOSQUITTO_PLUGIN_CLANG_EXTRA_ARGS";

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");
    // Tell cargo to invalidate the built crate whenever the extra args variable changes
    println!(
        "cargo:rerun-if-env-changed={}",
        MOSQUITTO_PLUGIN_CLANG_EXTRA_ARGS
    );

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let builder = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Filter functions with mosquitto_.*
        .allowlist_function("mosquitto_.*")
        // Filter types with mosquitto_.*
        .allowlist_type("mosquitto_.*")
        // Filter variables with MOSQ_.*
        .allowlist_var("MOSQ_.*")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    // If the clang extra arguments variable is set, add the arguments
    let builder = if let Ok(args) = env::var(MOSQUITTO_PLUGIN_CLANG_EXTRA_ARGS) {
        builder.clang_args(args.split_whitespace())
    } else {
        builder
    };

    let bindings = builder
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
