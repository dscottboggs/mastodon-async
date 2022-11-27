#[cfg(feature = "toml")]
extern crate skeptic;

#[cfg(feature = "toml")]
fn main() {
    skeptic::generate_doc_tests(&["README.md"]);
}

#[cfg(not(feature = "toml"))]
fn main() {
    // tests/skeptic.rs still expects a file to be at OUT_DIR/skeptic-tests.rs, so
    // make a dummy one
    use std::{env, fs::OpenOptions, path::Path};
    let out_dir = Path::new(&env::var("OUT_DIR").expect("no out_dir set")).join("skeptic-tests.rs");
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(out_dir)
        .expect("couldn't write to file");
}
