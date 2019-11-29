use std::fs::File;
use std::io::Cursor;
use std::path::PathBuf;

use roselib::files::STB;
use roselib::io::RoseFile;

#[test]
fn read_stb() {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("tests");
    root.push("data");

    //let file = root.join("list_quest.stb");
    let file = root.join("list_zone.stb");
    let stb = STB::from_path(&file).unwrap();

    assert_eq!(stb.identifier, "STB1");
    assert_eq!(stb.headers.len(), 38);
    assert_eq!(stb.rows(), 121);
    assert_eq!(stb.cols(), 38);

    for row in stb.data {
        assert_eq!(row.len(), 38);
    }
}

#[test]
fn write_stb() {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("tests");
    root.push("data");

    let stb_file = root.join("list_zone.stb");

    let f = File::open(&stb_file).unwrap();
    let stb_size = f.metadata().unwrap().len();

    let mut orig_stb = STB::from_path(&stb_file).unwrap();

    let mut buffer: Vec<u8> = Vec::new();
    buffer.resize(stb_size as usize, 0u8);

    let mut cursor = Cursor::new(buffer);
    orig_stb.write(&mut cursor).unwrap();

    cursor.set_position(0);

    let mut new_stb = STB::new();
    new_stb.read(&mut cursor).unwrap();

    assert_eq!(orig_stb.identifier, new_stb.identifier);
    assert_eq!(orig_stb.headers.len(), new_stb.headers.len());
    assert_eq!(orig_stb.data.len(), new_stb.data.len());
    assert_eq!(orig_stb, new_stb);
}
