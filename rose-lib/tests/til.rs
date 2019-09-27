extern crate roselib;

use roselib::files::TIL;
use roselib::io::RoseFile;
use std::path::PathBuf;

#[test]
fn read_til() {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("tests");
    root.push("data");

    let file = root.join("31_30.TIL");
    let til = TIL::from_path(&file).unwrap();

    assert_eq!(til.tiles.len(), 16);
    for t in til.tiles {
        assert_eq!(t.len(), 16);
    }
}
