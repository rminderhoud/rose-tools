use std::f32;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::iter;
use std::path::{Path, PathBuf};
use std::process::exit;

use clap::{crate_authors, crate_version, App, AppSettings, Arg, ArgMatches, SubCommand};
use failure::{bail, Error};
use image::{GrayImage, ImageBuffer};
use serde::{Deserialize, Serialize};

use roselib::files::zon::ZoneTileRotation;
use roselib::files::*;
use roselib::io::{RoseFile, RoseReader};

use rose_conv::{FromCsv, ToCsv};
use rose_conv::{FromJson, ToJson};

const SERIALIZE_VALUES: [&'static str; 8] =
    ["idx", "lit", "stb", "stl", "wstb", "til", "zon", "zsc"];

const DESERIALIZE_VALUES: [&'static str; 5] = ["idx", "lit", "stb", "stl", "zsc"];

#[derive(Debug, Deserialize, Serialize)]
struct TilemapTile {
    layer1: i32,
    layer2: i32,
    rotation: ZoneTileRotation,
}

#[derive(Debug, Deserialize, Serialize)]
struct TilemapFile {
    textures: Vec<String>,
    tiles: Vec<TilemapTile>,
    tilemap: Vec<Vec<i32>>,
}

fn main() {
    let matches = App::new("ROSE Converter")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Convert ROSE Online files to/from various formats")
        .arg(
            Arg::with_name("out_dir")
                .help("Directory to output converted files")
                .default_value("./out/")
                .short("o")
                .global(true),
        )
        .settings(&[
            AppSettings::SubcommandRequiredElseHelp,
            AppSettings::VersionlessSubcommands,
            AppSettings::DeriveDisplayOrder,
        ])
        .subcommand(
            SubCommand::with_name("map")
                .about("Convert ROSE map files")
                .arg(
                    Arg::with_name("map_dir")
                        .help("Map directory containing zon, him, til and ifo files")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("serialize")
                .visible_alias("se")
                .about("Serialize a ROSE File into JSON (CSV for STB/STL).")
                .arg(
                    Arg::with_name("input")
                        .help("Path to ROSE file")
                        .required(true),
                )
                .arg(
                    Arg::with_name("type")
                        .help("Type of file")
                        .required(false)
                        .short("t")
                        .long("type")
                        .takes_value(true)
                        .possible_values(&SERIALIZE_VALUES),
                ),
        )
        .subcommand(
            SubCommand::with_name("deserialize")
                .visible_alias("de")
                .about("Deserialize a ROSE file from JSON (CSV for STB/STL).")
                .arg(
                    Arg::with_name("type")
                        .help("ROSE file type")
                        .case_insensitive(true)
                        .possible_values(&DESERIALIZE_VALUES)
                        .required(true),
                )
                .arg(
                    Arg::with_name("input")
                        .help("Path to JSON/CSV file")
                        .required(true),
                ),
        )
        .get_matches();

    // Setup output directory
    let out_dir = Path::new(matches.value_of("out_dir").unwrap());
    if let Err(e) = fs::create_dir_all(&out_dir) {
        eprintln!(
            "Error creating output directory {}: {}",
            out_dir.to_str().unwrap_or(""),
            e
        );
        exit(1);
    }

    // Run subcommands
    let res = match matches.subcommand() {
        ("map", Some(matches)) => convert_map(matches),
        ("serialize", Some(matches)) => serialize(matches),
        ("deserialize", Some(matches)) => deserialize(matches),
        _ => {
            eprintln!("ROSE Online Converter. Run with `--help` for more info.");
            exit(1);
        }
    };

    if let Err(e) = res {
        eprintln!("Error occured: {}", e);
    }
}

fn serialize(matches: &ArgMatches) -> Result<(), Error> {
    let out_dir = Path::new(matches.value_of("out_dir").unwrap_or_default());
    let input = Path::new(matches.value_of("input").unwrap_or_default());
    let input_type = matches.value_of("type").unwrap_or_default();

    if !input.exists() {
        bail!("File does not exist: {}", input.display());
    }

    let extension = input
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_lowercase();

    let rose_type = if input_type.is_empty() {
        if !SERIALIZE_VALUES.contains(&extension.as_str()) {
            bail!("No type provided and unrecognized extension");
        }
        String::from(&extension)
    } else {
        String::from(input_type)
    };

    let out = out_dir
        .join(input.file_name().unwrap_or_default())
        .with_extension("");

    let data = match rose_type.as_str() {
        "stb" => STB::from_path(&input)?.to_csv()?,
        "stl" => STL::from_path(&input)?.to_csv()?,
        "idx" => IDX::from_path(&input)?.to_json()?,
        "lit" => LIT::from_path(&input)?.to_json()?,
        "til" => TIL::from_path(&input)?.to_json()?,
        "zon" => ZON::from_path(&input)?.to_json()?,
        "zsc" => ZSC::from_path(&input)?.to_json()?,
        "wstb" => {
            let f = File::open(input)?;
            let mut reader = RoseReader::new(f);
            reader.set_wide_strings(true);
            let mut stb: STB = RoseFile::new();
            stb.read(&mut reader)?;
            stb.to_csv()?
        }
        _ => bail!("Unsupported file type: {}", rose_type.as_str()),
    };

    let extension = if rose_type == "stb" || rose_type == "stl" {
        "csv"
    } else {
        "json"
    };

    let out = out.with_extension(extension);
    let mut f = File::create(&out)?;
    f.write_all(data.as_bytes())?;

    Ok(())
}

fn deserialize(matches: &ArgMatches) -> Result<(), Error> {
    let out_dir = Path::new(matches.value_of("out_dir").unwrap_or_default());
    let filetype = matches.value_of("type").unwrap_or_default();
    let input = Path::new(matches.value_of("input").unwrap_or_default());

    if !input.exists() {
        bail!("File does not exist: {}", input.display());
    }

    let out = out_dir
        .join(input.file_name().unwrap_or_default())
        .with_extension(filetype);

    let mut data = String::new();

    let mut file = File::open(&input)?;
    file.read_to_string(&mut data)?;

    match filetype {
        "stb" => STB::from_csv(&data)?.write_to_path(&out)?,
        "stl" => stl::from_csv(&data)?.write_to_path(&out)?,
        "idx" => IDX::from_json(&data)?.write_to_path(&out)?,
        "lit" => IDX::from_json(&data)?.write_to_path(&out)?,
        "zsc" => IDX::from_json(&data)?.write_to_path(&out)?,
        _ => bail!("Unsupported file type: {}", filetype),
    }

    Ok(())
}

/// Convert map files:
/// - ZON: JSON
/// - TIL: Combined into 1 JSON file
/// - IFO: Combined into 1 JSON file
/// - HIM: Combined into 1 greyscale png
fn convert_map(matches: &ArgMatches) -> Result<(), Error> {
    let map_dir = Path::new(matches.value_of("map_dir").unwrap());
    if !map_dir.is_dir() {
        bail!("Map path is not a directory: {:?}", map_dir);
    }

    println!("Loading map from: {}", map_dir.to_str().unwrap());

    // Collect coordinates from file names (using HIM as reference)
    let mut x_coords: Vec<u32> = Vec::new();
    let mut y_coords: Vec<u32> = Vec::new();

    for f in fs::read_dir(map_dir)? {
        let f = f?;
        let fpath = f.path();
        if !fpath.is_file() {
            continue;
        }

        if fpath.extension().unwrap().to_str().unwrap().to_lowercase() == "him" {
            let fname = fpath.file_stem().unwrap().to_str().unwrap();
            let parts: Vec<&str> = fname.split('_').collect();
            x_coords.push(parts[0].parse()?);
            y_coords.push(parts[1].parse()?);
        }
    }

    x_coords.sort();
    y_coords.sort();

    let x_min = *x_coords.iter().min().unwrap();
    let x_max = *x_coords.iter().max().unwrap();
    let y_min = *y_coords.iter().min().unwrap();
    let y_max = *y_coords.iter().max().unwrap();

    let map_width = (x_max - x_min + 1) * 65;
    let map_height = (y_max - y_min + 1) * 65;

    let mut max_height = f32::NAN;
    let mut min_height = f32::NAN;

    // Ensure map dimensions are divisible by 4 for tiling
    let new_map_width = (map_width as f32 / 4.0).ceil() * 4.0;
    let new_map_height = (map_height as f32 / 4.0).ceil() * 4.0;

    let new_map_width = new_map_width as u32 + 1;
    let new_map_height = new_map_height as u32 + 1;

    let mut heights: Vec<Vec<f32>> = Vec::new();
    heights.resize(
        new_map_height as usize,
        iter::repeat(0.0).take(new_map_width as usize).collect(),
    );

    // Number of tiles in x and y direction
    let tiles_x = new_map_width / 4;
    let tiles_y = new_map_height / 4;

    let mut tiles: Vec<Vec<i32>> = Vec::new();
    tiles.resize(
        tiles_y as usize,
        iter::repeat(0).take(tiles_x as usize).collect(),
    );

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            //-- Load HIMs
            let him_name = format!("{}_{}.HIM", x, y);
            let him_path = map_dir.join(&him_name);

            let him = HIM::from_path(&him_path).unwrap();
            if him.height != 65 || him.width != 65 {
                bail!(
                    "Unexpected HIM dimensions. Expected 65x65: {} ({}x{})",
                    &him_path.to_str().unwrap_or(&him_name),
                    him.width,
                    him.height
                );
            }

            for h in 0..him.height {
                for w in 0..him.width {
                    let height = him.heights[h as usize][w as usize];

                    if (height > max_height) || (max_height.is_nan()) {
                        max_height = height;
                    }
                    if (height < min_height) || (min_height.is_nan()) {
                        min_height = height;
                    }

                    let new_x = ((x - x_min) * 65) + w as u32;
                    let new_y = ((y - y_min) * 65) + h as u32;

                    heights[new_y as usize][new_x as usize] = height;
                }
            }

            // -- Load TILs
            let til_name = format!("{}_{}.TIL", x, y);
            let til_path = map_dir.join(&til_name);

            let til = TIL::from_path(&til_path).unwrap();
            if til.height != 16 || til.width != 16 {
                bail!(
                    "Unexpected TIL dimensions. Expected 16x16: {} ({}x{})",
                    &til_path.to_str().unwrap_or(&til_name),
                    til.width,
                    til.height
                );
            }

            for h in 0..til.height {
                for w in 0..til.width {
                    let tile_id = til.tiles[h as usize][w as usize].tile_id;

                    let new_x = ((x - x_min) * 16) + w as u32;
                    let new_y = ((y - y_min) * 16) + h as u32;

                    tiles[new_y as usize][new_x as usize] = tile_id;
                }
            }

            // TODO:
            // Load IFO data
        }
    }

    let map_name = map_dir.file_name().unwrap().to_str().unwrap();
    let out_dir = Path::new(matches.value_of("out_dir").unwrap_or("out"));

    // -- Heightmap image
    let delta_height = max_height - min_height;

    let mut height_image: GrayImage = ImageBuffer::new(new_map_width, new_map_height);

    for y in 0..new_map_height {
        for x in 0..new_map_width {
            let height = heights[y as usize][x as usize];

            let norm_height = |h| (255.0 * ((h - min_height) / delta_height)) as u8;

            let pixel = image::Luma([norm_height(height)]);
            height_image.put_pixel(x, y, pixel);
        }
    }

    // Save heightmap image
    let mut height_file = PathBuf::from(out_dir);
    height_file.push(map_name);
    height_file.set_extension("png");

    println!("Saving heightmap to: {}", &height_file.to_str().unwrap());
    height_image.save(height_file)?;

    // Dump ZON as JSON
    let zon = ZON::from_path(&map_dir.join(format!("{}.ZON", map_name)))?;
    let mut zon_file = PathBuf::from(out_dir);
    zon_file.push(map_name.to_string());
    zon_file.set_extension("json");

    println!("Dumping ZON file to: {}", &zon_file.to_str().unwrap());
    let f = File::create(zon_file)?;
    serde_json::to_writer_pretty(f, &zon)?;

    // Create tilemap file
    let mut tilemap_tiles: Vec<TilemapTile> = Vec::new();
    for zon_tile in zon.tiles {
        tilemap_tiles.push(TilemapTile {
            layer1: zon_tile.layer1 + zon_tile.offset1,
            layer2: zon_tile.layer2 + zon_tile.offset2,
            rotation: zon_tile.rotation,
        });
    }

    let tilemap = TilemapFile {
        textures: zon.textures,
        tiles: tilemap_tiles,
        tilemap: tiles,
    };

    let mut tile_file = PathBuf::from(out_dir);
    tile_file.push(format!("{}_tilemap", map_name));
    tile_file.set_extension("json");

    println!("Saving tilemap file to: {}", &tile_file.to_str().unwrap());
    let f = File::create(tile_file)?;
    serde_json::to_writer_pretty(f, &tilemap)?;

    // EXPORT IFO data as JSON

    Ok(())
}

/*
fn zms_to_obj(input: File, output: File) -> Result<(), Error> {
    let mut writer = BufWriter::new(output);

    //let z = ZMS::from_reader(&mut reader)?;
    let z = ZMS::from_file(&input)?;

    writer
        .write(format!("# Exported using {} v{} ({})\n",
                       env!("CARGO_PKG_NAME"),
                       env!("CARGO_PKG_VERSION"),
                       env!("CARGO_PKG_HOMEPAGE"))
                       .as_bytes())?;

    // -- Write vertex data
    for v in &z.vertices {
        writer
            .write(format!("v {} {} {}\n", v.position.x, v.position.y, v.position.z).as_bytes())?;
    }

    for v in &z.vertices {
        writer
            .write(format!("vt {} {}\n", v.uv1.x, 1.0 - v.uv1.y).as_bytes())?;
    }

    for v in &z.vertices {
        writer
            .write(format!("vn {} {} {}\n", v.normal.x, v.normal.y, v.normal.z).as_bytes())?;
    }

    // -- Write face data
    for i in z.indices {
        writer
            .write(format!("f {x}/{x}/{x} {y}/{y}/{y} {z}/{z}/{z}\n",
                           x = i.x + 1,
                           y = i.y + 1,
                           z = i.z + 1)
                           .as_bytes())?;
    }

    Ok(())
}
*/
