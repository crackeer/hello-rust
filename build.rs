
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
}
