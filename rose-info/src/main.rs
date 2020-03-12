use roselib::files::TIL;
use roselib::files::ZON;
use roselib::io::RoseFile;
use std::collections::HashMap;
use std::path::Path;

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() <= 2 {
        println!("Usage: rose-info <command> <paths...>");
        println!("Commands: til_brushes, zon_brushes");
        ::std::process::exit(1);
    }
    let cmd = args.remove(0);
    match cmd.as_str() {
        "til_brushes" => til_brush_info(args.as_slice()),
        "zon_brushes" => zon_brush_info(args.as_slice()),
        _ => {
            eprintln!("Command not recognized: {0}", cmd);
            ::std::process::exit(1);
        }
    }
}

fn til_brush_info(paths: &[String]) {
    let mut map_brushes: HashMap<&Path, Vec<u8>> = HashMap::new();

    let mut max_map_path = "";
    let mut max_map_brush_count = 0;

    for til_path in paths {
        let til_path = Path::new(til_path);
        let til_parent = til_path.parent().unwrap();
        let til = TIL::from_path(&til_path).expect("Invalid TIL file");

        for tile_row in til.tiles {
            for tile in tile_row {
                if map_brushes.contains_key(til_parent) {
                    if let Some(v) = map_brushes.get_mut(til_parent) {
                        v.push(tile.brush_id);
                    }
                } else {
                    map_brushes.insert(til_parent, vec![tile.brush_id]);
                }
            }
        }
    }

    for (map_path, brushes) in map_brushes.iter_mut() {
        brushes.sort_unstable();
        brushes.dedup();
        let brush_count = brushes.len();
        if brush_count > max_map_brush_count {
            max_map_brush_count = brush_count;
            max_map_path = map_path.to_str().unwrap();
        }

        println!(
            "{} brushes used in {}",
            brush_count,
            map_path.to_str().unwrap()
        );
    }

    println!(
        "MAX: {} brushes used in {}",
        max_map_brush_count, max_map_path,
    );
}

fn zon_brush_info(paths: &[String]) {
    let mut max_brushes = 0;
    let mut max_brush_path = "";

    for zon_path in paths {
        let zon = ZON::from_path(Path::new(&zon_path)).expect("Invalid ZON file");
        let brushes = zon.tiles.len();
        println!("{} brushes in {}", brushes, &zon_path);

        if brushes > max_brushes {
            max_brushes = brushes;
            max_brush_path = zon_path;
        }
    }

    println!("MAX: {} brushes in {}", max_brushes, max_brush_path);
}
