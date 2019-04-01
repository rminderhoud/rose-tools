extern crate roselib;

use std::cmp::{min, max};
use std::env;
use std::f32;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use image::ImageBuffer;
use roselib::files::HIM;

// Default values in legacy files
const CHUNK_WIDTH: i32 = 65;
const CHUNK_HEIGHT: i32 = 65;

#[derive(Debug)]
struct Vec2 {
    x: i32,
    y: i32,
}

fn get_coords_from_path(path: &Path) -> Vec2 {
    let fname = path.file_stem().unwrap().to_str().unwrap();
    let c: Vec<i32> = fname
        .to_string()
        .split('_')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    Vec2 { x: c[0], y: c[1] }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: him_paths_to_image <hims directory>");
        ::std::process::exit(1);
    }

    let path = Path::new(args[1].as_str());

    if !path.is_dir() {
        println!("{} is not a directory", args[1]);
        ::std::process::exit(1);
    }

    let mut him_paths = Vec::new();
    for entry in path.read_dir().expect("Unable to read directory contents") {
        if let Ok(entry) = entry {
            if let Some(e) = entry.path().extension() {
                let ext = e.to_str().unwrap().to_lowercase();
                if ext == "him" {
                    him_paths.push(entry.path());
                }
            }
        }
    }

    if him_paths.len() == 0 {
        println!("No him files detected, please use .him extension");
        ::std::process::exit(1);
    }

    // Get chunk coordinate range
    let mut chunk_min = Vec2 { x: 999, y: 999 };
    let mut chunk_max = Vec2 { x: -1, y: -1 };
    let mut height_max = f32::NAN;
    let mut height_min = f32::NAN;

    for him_path in &him_paths {
        let coords = get_coords_from_path(him_path);

        chunk_min.x = min(coords.x, chunk_min.x);
        chunk_min.y = min(coords.y, chunk_min.y);

        chunk_max.x = max(coords.x, chunk_max.x);
        chunk_max.y = max(coords.y, chunk_max.y);

        let f = File::open(him_path).unwrap();
        let mut buff = BufReader::new(f);
        let mut chunk = HIM::new();
        chunk.read(&mut buff);

        if height_min.is_nan() || chunk.min_height < height_min {
            height_min = chunk.min_height;
        }

        if height_max.is_nan() || chunk.max_height > height_max {
            height_max = chunk.max_height;
        }
    }

    let mut image: image::RgbImage = ImageBuffer::new(512, 512);

    for him_path in &him_paths {
        let f = File::open(him_path).unwrap();
        let mut buff = BufReader::new(f);
        let mut chunk = HIM::new();
        chunk.read(&mut buff);

        if chunk.width != CHUNK_WIDTH || chunk.height != CHUNK_HEIGHT {
            let s = him_path.to_str().expect("Error with him path string");
            println!("Invalid chunk width/height detected: {}", s);
            ::std::process::exit(1);
        }

        let coords = get_coords_from_path(him_path);
        let chunk_offset = Vec2 {
            x: coords.x - chunk_min.x,
            y: coords.y - chunk_min.y,
        };

        for y in 0..chunk.width {
            for x in 0..chunk.height {
                let h = y as usize;
                let w = x as usize;

                // Normalize height so lowest point is at 0
                let height = chunk.heights[h][w];
                let norm_height = height + (-1.0 * height_min);

                if norm_height > 65535.0 {
                    println!("Tall terrains not supported (65535 max height diff)");
                    ::std::process::exit(1);
                }

                let image_x = ((chunk_offset.x * CHUNK_WIDTH) + x) as u32;
                let image_y = ((chunk_offset.y * CHUNK_HEIGHT) + y) as u32;


                let redgreen = norm_height.trunc() as u16;
                let red = (redgreen >> 8) as u8; // MSB
                let green = redgreen as u8; // LSB

                image.put_pixel(image_x, image_y, image::Rgb([red, green, 0]));
            }
        }
    }

    image.save(Path::new("out.png"));
}
