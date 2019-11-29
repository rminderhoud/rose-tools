use std::fs::File;
use std::io::Cursor;
use std::path::PathBuf;

use roselib::files::IDX;
use roselib::io::RoseFile;

#[test]
fn write_idx() {
    let mut idx_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    idx_path.push("tests");
    idx_path.push("data");
    idx_path.push("data.idx");

    let idx = IDX::from_path(&idx_path).unwrap();

    assert_eq!(idx.base_version, 129);
    assert_eq!(idx.current_version, 129);
    assert_eq!(idx.file_systems.len(), 2);

    let ref data_vfs = idx.file_systems[0];
    let ref data_vfs_last = data_vfs.files[data_vfs.files.len() - 1];

    assert_eq!(data_vfs.filename.to_str().unwrap(), "DATA.VFS");
    assert_eq!(data_vfs.files.len(), 3193);
    assert_eq!(
        data_vfs_last.filepath.to_str().unwrap(),
        "3DDATA/EFFECT/_YETITYRANT_SKILL_01.EFT"
    );

    let ref map_vfs = idx.file_systems[1];
    let ref map_vfs_last = map_vfs.files[map_vfs.files.len() - 1];

    assert_eq!(map_vfs.filename.to_str().unwrap(), "MAP.VFS");
    assert_eq!(map_vfs.files.len(), 11053);
    assert_eq!(
        map_vfs_last.filepath.to_str().unwrap(),
        "3DDATA/TERRAIN/TILES/ZONETYPEINFO.STB"
    );
}

#[test]
fn read_idx() {
    let mut idx_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    idx_path.push("tests");
    idx_path.push("data");
    idx_path.push("data.idx");

    let f = File::open(&idx_path).unwrap();
    let idx_size = f.metadata().unwrap().len();

    let mut orig_idx = IDX::from_file(&f).unwrap();

    let mut buffer: Vec<u8> = Vec::new();
    buffer.resize(idx_size as usize, 0u8);

    let mut cursor = Cursor::new(buffer);
    orig_idx.write(&mut cursor).unwrap();

    // Load again to check save was successful
    cursor.set_position(0);
    let mut new_idx = IDX::new();
    new_idx.read(&mut cursor).unwrap();

    assert_eq!(new_idx.base_version, 129);
    assert_eq!(new_idx.current_version, 129);
    assert_eq!(new_idx.file_systems.len(), 2);

    let ref data_vfs = new_idx.file_systems[0];
    let ref data_vfs_last = data_vfs.files[data_vfs.files.len() - 1];

    assert_eq!(data_vfs.filename.to_str().unwrap(), "DATA.VFS");
    assert_eq!(data_vfs.files.len(), 3193);
    assert_eq!(
        data_vfs_last.filepath.to_str().unwrap(),
        "3DDATA/EFFECT/_YETITYRANT_SKILL_01.EFT"
    );

    let ref map_vfs = new_idx.file_systems[1];
    let ref map_vfs_last = map_vfs.files[map_vfs.files.len() - 1];

    assert_eq!(map_vfs.filename.to_str().unwrap(), "MAP.VFS");
    assert_eq!(map_vfs.files.len(), 11053);
    assert_eq!(
        map_vfs_last.filepath.to_str().unwrap(),
        "3DDATA/TERRAIN/TILES/ZONETYPEINFO.STB"
    );
}
