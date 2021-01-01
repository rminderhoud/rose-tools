use std::path::PathBuf;

use roselib::files::IFO;
use roselib::io::RoseFile;
use roselib::utils::Vector2;

#[test]
fn read_ifo() {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("tests");
    root.push("data");

    let file = root.join("31_30.ifo");
    let ifo = IFO::from_path(&file).unwrap();
    assert_eq!(ifo.map_pos, Vector2::<i32> { x: 16, y: 16 });
    assert_eq!(ifo.zone_pos, Vector2::<i32> { x: 31, y: 30 });
    assert_eq!(ifo.objects.len(), 73);
    assert_eq!(ifo.npcs.len(), 1);
    assert_eq!(ifo.sounds.len(), 0);
    assert_eq!(ifo.effects.len(), 1);
    assert_eq!(ifo.animations.len(), 0);
    assert_eq!(ifo.waters.len(), 16);
    assert_eq!(ifo.buildings.len(), 1);
    assert_eq!(ifo.warps.len(), 0);
    assert_eq!(ifo.oceans.len(), 1);
    assert_eq!(ifo.monster_spawns.len(), 35);
    assert_eq!(ifo.collision_objects.len(), 3);
    assert_eq!(ifo.events.len(), 1);

    let file = root.join("34_30.ifo");
    let ifo = IFO::from_path(&file).unwrap();
    assert_eq!(ifo.map_pos, Vector2::<i32> { x: 16, y: 16 });
    assert_eq!(ifo.zone_pos, Vector2::<i32> { x: 34, y: 30 });
    assert_eq!(ifo.objects.len(), 23);
    assert_eq!(ifo.npcs.len(), 0);
    assert_eq!(ifo.sounds.len(), 0);
    assert_eq!(ifo.effects.len(), 0);
    assert_eq!(ifo.animations.len(), 0);
    assert_eq!(ifo.waters.len(), 16);
    assert_eq!(ifo.buildings.len(), 0);
    assert_eq!(ifo.warps.len(), 1);
    assert_eq!(ifo.oceans.len(), 1);
    assert_eq!(ifo.monster_spawns.len(), 0);
    assert_eq!(ifo.collision_objects.len(), 0);
    assert_eq!(ifo.events.len(), 0);
}
