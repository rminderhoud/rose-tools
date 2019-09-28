extern crate roselib;

use std::fs::File;
use std::io::Cursor;
use std::path::PathBuf;

use roselib::files::ZMO;
use roselib::io::RoseFile;

#[test]
fn read_zmo() {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("tests");
    root.push("data");

    {
        let zmo_path = root.join("empty_walk_m1.zmo");
        let zmo = ZMO::from_path(&zmo_path).unwrap();
        assert_eq!(zmo.identifier, "ZMO0002");
        assert_eq!(zmo.fps, 30);
        assert_eq!(zmo.frames, 27);
        assert_eq!(zmo.channels.len(), 22);
    }

    {
        let zmo_path = root.join("eluxsamtower.zmo");
        let zmo = ZMO::from_path(&zmo_path).unwrap();
        assert_eq!(zmo.identifier, "ZMO0002");
        assert_eq!(zmo.fps, 30);
        assert_eq!(zmo.frames, 11);
        assert_eq!(zmo.channels.len(), 216);
    }

    {
        let zmo_path = root.join("item_ani.zmo");
        let zmo = ZMO::from_path(&zmo_path).unwrap();
        assert_eq!(zmo.identifier, "ZMO0002");
        assert_eq!(zmo.fps, 30);
        assert_eq!(zmo.frames, 61);
        assert_eq!(zmo.channels.len(), 2);
    }

    {
        let zmo_path = root.join("_wind_01.zmo");
        let zmo = ZMO::from_path(&zmo_path).unwrap();
        assert_eq!(zmo.identifier, "ZMO0002");
        assert_eq!(zmo.fps, 30);
        assert_eq!(zmo.frames, 47);
        assert_eq!(zmo.channels.len(), 2196);
    }
}

#[test]
fn write_zmo() {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("tests");
    root.push("data");

    let file1 = root.join("empty_walk_m1.zmo");
    let file2 = root.join("eluxsamtower.zmo");
    let file3 = root.join("item_ani.zmo");
    let file4 = root.join("_wind_01.zmo");

    for zmo_file in [file1, file2, file3, file4].iter() {
        let f = File::open(&zmo_file).unwrap();
        let zmo_size = f.metadata().unwrap().len();

        let mut orig_zmo = ZMO::from_path(&zmo_file).unwrap();

        let mut buffer: Vec<u8> = Vec::new();
        buffer.resize(zmo_size as usize, 0u8);

        let mut cursor = Cursor::new(buffer);
        orig_zmo.write(&mut cursor).unwrap();

        cursor.set_position(0);

        let mut new_zmo = ZMO::new();
        new_zmo.read(&mut cursor).unwrap();

        assert_eq!(orig_zmo, new_zmo);
    }
}
