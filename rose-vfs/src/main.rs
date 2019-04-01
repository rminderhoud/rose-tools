#[macro_use]
extern crate clap;
extern crate roselib;

use std::ffi::OsStr;
use std::fs::{File, create_dir_all};
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::process::exit;
use roselib::files::IDX;
use roselib::io::RoseFile;

fn main() {
    let yaml = load_yaml!("vfs_extractor.yaml");
    let matches = clap::App::from_yaml(yaml).get_matches();

    let dry_run = matches.is_present("dry_run");
    let flat = matches.is_present("flat");
    let verbose = matches.is_present("verbose");

    let out_dir_str = matches.value_of("out_dir").unwrap();
    let out_dir = Path::new(out_dir_str);

    let idx_path_str = matches.value_of("idx").unwrap();
    let idx_path = Path::new(idx_path_str);

    let include: Vec<String> = match matches.values_of("include") {
        Some(v) => v.map(|s| s.to_lowercase()).collect(),
        None => Vec::new(), 
    };

    let idx_file = match File::open(&idx_path) {
        Ok(f) => f,
        Err(e) => {
            println!("Error opening idx file: {}", e);
            exit(1);
        }
    };

    let idx = match IDX::from_file(&idx_file) {
        Ok(i) => i,
        Err(e) => {
            println!("Error reading idx file: {}", e);
            exit(1);
        }
    };

    println!("File loaded: {:?}", idx_path.file_name().unwrap());
    println!("Version: {}_{}", idx.base_version, idx.current_version);

    let idx_path_dir = Path::new(idx_path.parent().unwrap());

    for fs in idx.file_systems {
        let mut vfs_path = PathBuf::from(idx_path_dir);
        vfs_path.push(&fs.filename);

        let mut vfs = match File::open(vfs_path) { 
            Ok(f) => f,
            Err(e) => {
                println!("Unable to open {}: {}",
                         &fs.filename.to_str().unwrap_or(""),
                         e);
                continue;
            }
        };

        println!("Loaded {}: {} files indexed",
                 fs.filename.to_str().unwrap_or(""),
                 fs.files.len());

        let mut extracted = 0;
        for file in fs.files {
            let file_ext = file.filepath
                .extension()
                .unwrap_or(OsStr::new(""))
                .to_str()
                .unwrap_or("");
            if include.is_empty() | include.contains(&file_ext.to_lowercase()) {
                if verbose {
                    println!("Extracting: {}", file.filepath.to_str().unwrap_or(""));
                }

                let mut out_file_path = PathBuf::from(out_dir);
                if flat {
                    out_file_path.push(&file.filepath.file_name().unwrap());
                } else {
                    out_file_path.push(&file.filepath);
                }

                let out_file_parent = out_file_path.parent().unwrap();
                if !out_file_parent.exists() {
                    if !dry_run {
                        if let Err(e) = create_dir_all(out_file_parent) {
                            println!("Error creating output directory: {}", e);
                            continue;
                        };
                    }
                }

                if !dry_run {
                    let mut out_file = match File::create(&out_file_path) {
                        Ok(f) => f,
                        Err(e) => {
                            println!("Unable to write file {}: {}",
                                     out_file_path.to_str().unwrap(),
                                     e);
                            continue;
                        }
                    };

                    let mut buffer: Vec<u8> = Vec::new();
                    buffer.resize(file.size as usize, 0u8);
                    if let Err(e) = vfs.seek(SeekFrom::Start(file.offset as u64)) {
                        println!("Error reading data from {}: {}",
                                 fs.filename.to_str().unwrap(),
                                 e);
                        continue;
                    }

                    if let Err(e) = vfs.read_exact(&mut buffer) {
                        println!("Error reading data from {}: {}",
                                 fs.filename.to_str().unwrap(),
                                 e);
                        continue;
                    }

                    if let Err(e) = out_file.write_all(&buffer) {
                        println!("Error writing file {}: {}",
                                 out_file_path.file_name().unwrap().to_str().unwrap(),
                                 e);

                        continue;
                    }

                }

                extracted = extracted + 1;
            }
        }
        println!("{} files extracted", extracted);
    }
    exit(0);
}
