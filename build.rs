use std::{env, path::PathBuf};

fn main() {
    println!("hello, build.rs");
    println!(
        "cargo:rerun-if-changed=build.rs",
    );
    let library_name = "slm_runtime";
    println!(
        "cargo:rustc-link-search=native={}",
        &"/usr/local/share/senseshield/sdk/C/lib64"
    );
    println!("cargo:rustc-link-lib=static={}", library_name);
    /*
    println!("cargo:rerun-if-changed=ss_lm_runtime.h");
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("ss_lm_runtime.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    */
}
