use cbindgen;
use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
      .with_crate(crate_dir)
      .with_namespace("Rose")
      .with_parse_deps(true)
      .with_parse_include(&["roselib"])
      .generate()
      .expect("Unable to generate bindings")
      .write_to_file("bindings.h");
}