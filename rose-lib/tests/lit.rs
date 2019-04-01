extern crate roselib;

use std::fs::File;
use std::io::Cursor;
use std::path::PathBuf;

use roselib::files::LIT;
use roselib::io::RoseFile;

#[test]
fn read_lit() {
    let mut lit_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    lit_path.push("tests");
    lit_path.push("data");
    lit_path.push("OBJECTLIGHTMAPDATA.LIT");

    let lit = LIT::from_path(&lit_path).unwrap();

    assert_eq!(lit.objects.len(), 266);
    assert_eq!(lit.filenames.len(), 38);

    let ref first_obj = lit.objects[0];
    let ref first_part = first_obj.parts[0];
    let ref last_obj = lit.objects[lit.objects.len() - 1];
    let ref last_part = last_obj.parts[last_obj.parts.len() - 1];

    assert_eq!(first_obj.id, 1);
    assert_eq!(first_obj.parts.len(), 8);

    assert_eq!(first_part.name, "fountain_Object_1_0_32_32_LightingMap.tga");
    assert_eq!(first_part.id, 0);
    assert_eq!(first_part.filename, "Object_256_1.dds");
    assert_eq!(first_part.lightmap_index, 10);
    assert_eq!(first_part.pixels_per_part, 256);
    assert_eq!(first_part.parts_per_width, 2);
    assert_eq!(first_part.part_position, 2);

    assert_eq!(last_obj.id, 266);
    assert_eq!(last_obj.parts.len(), 1);

    assert_eq!(last_part.name,
               "stonewall03_Object_266_0_32_32_LightingMap.tga");
    assert_eq!(last_part.id, 0);
    assert_eq!(last_part.filename, "Object_32_0.dds");
    assert_eq!(last_part.lightmap_index, 0);
    assert_eq!(last_part.pixels_per_part, 32);
    assert_eq!(last_part.parts_per_width, 16);
    assert_eq!(last_part.part_position, 52);
}

#[test]
fn write_lit() {
    let mut lit_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    lit_path.push("tests");
    lit_path.push("data");
    lit_path.push("OBJECTLIGHTMAPDATA.LIT");

    let f = File::open(&lit_path).unwrap();
    let lit_size = f.metadata().unwrap().len();

    let mut orig_lit = LIT::from_path(&lit_path).unwrap();

    let mut buffer: Vec<u8> = Vec::new();
    buffer.resize(lit_size as usize, 0u8);

    let mut cursor = Cursor::new(buffer);
    orig_lit.write(&mut cursor).unwrap();

    cursor.set_position(0);
    let mut new_lit = LIT::new();
    new_lit.read(&mut cursor).unwrap();

    assert_eq!(new_lit.objects.len(), 266);
    assert_eq!(new_lit.filenames.len(), 38);

    let ref first_obj = new_lit.objects[0];
    let ref first_part = first_obj.parts[0];
    let ref last_obj = new_lit.objects[new_lit.objects.len() - 1];
    let ref last_part = last_obj.parts[last_obj.parts.len() - 1];

    assert_eq!(first_obj.id, 1);
    assert_eq!(first_obj.parts.len(), 8);

    assert_eq!(first_part.name, "fountain_Object_1_0_32_32_LightingMap.tga");
    assert_eq!(first_part.id, 0);
    assert_eq!(first_part.filename, "Object_256_1.dds");
    assert_eq!(first_part.lightmap_index, 10);
    assert_eq!(first_part.pixels_per_part, 256);
    assert_eq!(first_part.parts_per_width, 2);
    assert_eq!(first_part.part_position, 2);

    assert_eq!(last_obj.id, 266);
    assert_eq!(last_obj.parts.len(), 1);

    assert_eq!(last_part.name,
               "stonewall03_Object_266_0_32_32_LightingMap.tga");
    assert_eq!(last_part.id, 0);
    assert_eq!(last_part.filename, "Object_32_0.dds");
    assert_eq!(last_part.lightmap_index, 0);
    assert_eq!(last_part.pixels_per_part, 32);
    assert_eq!(last_part.parts_per_width, 16);
    assert_eq!(last_part.part_position, 52);
}
