use std::fs::File;
use std::io::Cursor;
use std::path::PathBuf;

use roselib::files::ZSC;
use roselib::files::zsc::*;
use roselib::io::RoseFile;
use roselib::utils::Color3;

#[test]
fn read_zsc() {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("tests");
    root.push("data");

    let file = root.join("part_npc.zsc");
    let zsc = ZSC::from_path(&file).unwrap();

    assert_eq!(zsc.meshes.len(), 862);
    assert_eq!(zsc.meshes[0], PathBuf::from(r#"3DData\\NPC\\animal\\larva\\larva1.ZMS"#));

    assert_eq!(zsc.materials.len(), 862);
    assert_eq!(zsc.materials[0].path, PathBuf::from(r#"3DData\\NPC\\animal\\larva\\larva1.dds"#));
    assert_eq!(zsc.materials[0].is_skin, true);
    assert_eq!(zsc.materials[0].alpha_enabled, false);
    assert_eq!(zsc.materials[0].two_sided, false);
    assert_eq!(zsc.materials[0].alpha_test_enabled, true);
    assert_eq!(zsc.materials[0].alpha_ref, 128);
    assert_eq!(zsc.materials[0].z_write_enabled, true);
    assert_eq!(zsc.materials[0].z_test_enabled, true);
    assert_eq!(zsc.materials[0].blend_mode, SceneBlendMode::None);
    assert_eq!(zsc.materials[0].specular_enabled, false);
    assert_eq!(zsc.materials[0].alpha, 1.0);
    assert_eq!(zsc.materials[0].glow_type, SceneGlowType::None);
    assert_eq!(zsc.materials[0].glow_color, Color3::rgb(1.0, 1.0, 1.0));

    assert_eq!(zsc.effects.len(), 0);
    assert_eq!(zsc.objects.len(), 586);
}

#[test]
fn write_zsc() {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("tests");
    root.push("data");

    let zsc_file = root.join("part_npc.zsc");

    let f = File::open(&zsc_file).unwrap();
    let zsc_size = f.metadata().unwrap().len();

    let mut orig_zsc = ZSC::from_path(&zsc_file).unwrap();

    let mut buffer: Vec<u8> = Vec::new();
    buffer.resize(zsc_size as usize, 0u8);

    let mut cursor = Cursor::new(buffer);
    orig_zsc.write(&mut cursor).unwrap();

    cursor.set_position(0);

    let mut new_zsc = ZSC::new();
    new_zsc.read(&mut cursor).unwrap();

    assert_eq!(orig_zsc, new_zsc);
}