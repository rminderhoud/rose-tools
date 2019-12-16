use std::fs::File;
use std::io::Cursor;
use std::path::PathBuf;

use roselib::files::TSI;
use roselib::io::RoseFile;

#[test]
fn read_tsi() {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("tests");
    root.push("data");

    let file = root.join("ui.tsi");
    let tsi = TSI::from_path(&file).unwrap();

    assert_eq!(tsi.sprite_sheets.len(), 44);

    for sheet in &tsi.sprite_sheets {
        assert_eq!(
            sheet
                .path
                .extension()
                .unwrap()
                .to_str()
                .unwrap()
                .to_lowercase(),
            "dds"
        );

        for sprite in &sheet.sprites {
            assert_ne!(sprite.name, "");
        }
    }

    assert_eq!(tsi.total_sprites(), 667);
}

#[test]
fn write_tsi() {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("tests");
    root.push("data");

    let tsi_file = root.join("ui.tsi");

    let f = File::open(&tsi_file).unwrap();
    let tsi_size = f.metadata().unwrap().len();

    let mut orig_tsi = TSI::from_path(&tsi_file).unwrap();

    let mut buffer: Vec<u8> = Vec::new();
    buffer.resize(tsi_size as usize, 0u8);

    let mut cursor = Cursor::new(buffer);
    orig_tsi.write(&mut cursor).unwrap();

    cursor.set_position(0);

    let mut new_tsi = TSI::new();
    new_tsi.read(&mut cursor).unwrap();

    assert_eq!(orig_tsi.sprite_sheets, new_tsi.sprite_sheets);
    assert_eq!(orig_tsi.total_sprites(), new_tsi.total_sprites());
}
