use std::fs::File;
use std::io::Cursor;
use std::path::PathBuf;

use roselib::files::stl::{StringTableLanguage, StringTableLanguageTable, StringTableType};
use roselib::files::STL;
use roselib::io::RoseFile;

#[test]
fn read_stl() {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("tests");
    root.push("data");

    let stl_data = [
        ("list_faceitem_s.stl", StringTableType::Item, 80),
        ("list_quest_s.stl", StringTableType::Quest, 243),
        ("str_itemtype.stl", StringTableType::Normal, 156),
    ];

    for (filename, format, row_count) in &stl_data {
        let file = root.join(filename);
        let stl = STL::from_path(&file).unwrap();

        assert_eq!(stl.format, *format);
        assert_eq!(stl.language_count(), 5);
        assert_eq!(stl.row_count(), *row_count as usize);

        for language_id in 0..stl.language_count() {
            let language = StringTableLanguage::from(language_id as u32);
            let table = &stl.language_tables[language_id];
            assert_eq!(table.language, language);
        }
    }
}

#[test]
fn write_stl() {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("tests");
    root.push("data");

    let stl_data = [
        "list_faceitem_s.stl",
        "list_quest_s.stl",
        "str_itemtype.stl",
    ];

    for filename in &stl_data {
        let stl_file = root.join(filename);

        let f = File::open(&stl_file).unwrap();
        let stl_size = f.metadata().unwrap().len();

        let mut orig_stl = STL::from_path(&stl_file).unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        buffer.resize(stl_size as usize, 0u8);

        let mut cursor = Cursor::new(buffer);
        orig_stl.write(&mut cursor).unwrap();
        cursor.set_position(0);

        let mut new_stl = STL::new();
        new_stl.read(&mut cursor).unwrap();

        assert_eq!(orig_stl.format, new_stl.format);
        assert_eq!(orig_stl.language_count(), new_stl.language_count());
        assert_eq!(orig_stl.row_count(), new_stl.row_count());
        assert_eq!(orig_stl, new_stl);
    }
}
