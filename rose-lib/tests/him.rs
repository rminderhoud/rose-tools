use std::path::PathBuf;

use roselib::files::HIM;
use roselib::io::RoseFile;

#[test]
fn read_him() {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("tests");
    root.push("data");

    let file = root.join("33_30.HIM");
    let him = HIM::from_path(&file).unwrap();
    assert_eq!(him.width, 65);
    assert_eq!(him.length, 65);
    assert_eq!(him.heights.len(), (him.width * him.length) as usize);
    assert_eq!(him.grid_count, 4);
    assert_eq!(him.scale, 250.0);
    assert_eq!(him.min_height, 0.0);
    assert_eq!(him.max_height, 5463.6577);
}
