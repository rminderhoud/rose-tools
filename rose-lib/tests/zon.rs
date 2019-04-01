extern crate roselib;

use std::path::PathBuf;
use roselib::files::ZON;
use roselib::files::zon::*;
use roselib::io::RoseFile;


#[test]
fn read_zon() {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("tests");
    root.push("data");

    let file = root.join("JGT01.ZON");
    let zon = ZON::from_path(&file).unwrap();

    assert_eq!(zon.zone_type, ZoneType::Grass);
    assert_eq!(zon.width, 64);
    assert_eq!(zon.height, 64);
    assert_eq!(zon.grid_count, 4);
    assert_eq!(zon.grid_size, 250.0);
    assert_eq!(zon.start_position.x, 32);
    assert_eq!(zon.start_position.y, 32);
    assert_eq!(zon.positions.len(), zon.height as usize);
    for pos in zon.positions {
        assert_eq!(pos.len(), zon.width as usize);
    }
    assert_eq!(zon.event_points.len(), 5);
    assert_eq!(zon.event_points[0].name, "start");
    assert_eq!(zon.event_points[1].name, "restore");
    assert_eq!(zon.event_points[2].name, "WARP-JD01-JG01_2");
    assert_eq!(zon.event_points[3].name, "WARP-JD01-JG01");
    assert_eq!(zon.event_points[4].name, "WARP-JZ01-JG01");

    assert_eq!(zon.textures.len(), 48);
    assert_eq!(zon.tiles.len(), 238);
    assert_eq!(zon.name, "0");
    assert_eq!(zon.is_underground, false);
    assert_eq!(zon.background_music, "button1");
    assert_eq!(zon.sky, "button2");
}
