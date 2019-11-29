use std::fs::File;
use std::io::Cursor;
use std::path::PathBuf;

use roselib::files::ZMD;
use roselib::io::RoseFile;

#[test]
fn read_zmd() {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("tests");
    root.push("data");

    let zmd_path = root.join("male.zmd");

    let skeleton = ZMD::from_path(&zmd_path).unwrap();
    assert_eq!(skeleton.bones.len(), 21);
    assert_eq!(skeleton.dummy_bones.len(), 7);
}

#[test]
fn write_zmd() {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("tests");
    root.push("data");

    let orig_zmd_path = root.join("male.zmd");

    let orig_zmd_file = File::open(&orig_zmd_path).unwrap();
    let orig_zmd_size = orig_zmd_file.metadata().unwrap().len();

    let mut orig_zmd = ZMD::from_path(&orig_zmd_path).unwrap();

    let mut buffer: Vec<u8> = Vec::new();
    buffer.resize(orig_zmd_size as usize, 0u8);

    let mut cursor = Cursor::new(buffer);
    orig_zmd.write(&mut cursor).unwrap();

    cursor.set_position(0);

    let mut new_zmd = ZMD::new();
    new_zmd.read(&mut cursor).unwrap();

    assert_eq!(orig_zmd, new_zmd);
}
