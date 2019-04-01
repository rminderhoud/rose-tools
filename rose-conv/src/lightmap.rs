#[macro_use]
extern crate clap;
extern crate serde_json;
extern crate roseon;

use std::fs::{File, create_dir_all};
use std::io::{BufReader, BufWriter};
use std::path::{PathBuf, Path};
use std::process::exit;
use clap::ArgMatches;
use roseon::lightmap::Lightmap;

fn encode(matches: &ArgMatches, out_dir: &Path) {
    let filenames: Vec<&str> = match matches.values_of("files") {
        Some(f) => f.collect(),
        None => {
            println!("Please provide a list of files");
            exit(1)
        }
    };

    for filename in filenames {
        let in_file = match File::open(filename) {
            Ok(f) => f,
            Err(e) => {
                println!("Unable to open {}: {}", &filename, e);
                continue;
            }
        };

        let reader = BufReader::new(in_file);
        let mut lit: Lightmap = match serde_json::from_reader(reader) {
            Ok(l) => l,
            Err(e) => {
                println!("Error reading JSON file {}: {}", &filename, e);
                continue;
            }
        };

        let mut out_file_path = PathBuf::from(out_dir);
        out_file_path.push(filename);
        out_file_path.set_extension("lit");

        let out_file_parent = out_file_path.parent().unwrap();
        if !out_file_parent.exists() {
            if let Err(e) = create_dir_all(out_file_parent) {
                println!("Error creating output directory: {}", e);
                continue;
            };
        }

        let out_file = match File::create(&out_file_path) {
            Ok(f) => f,
            Err(e) => {
                println!("Unable to open {}: {}", &out_file_path.to_str().unwrap(), e);
                continue;
            }
        };

        if let Err(e) = lit.save(out_file) {
            println!("Error saving file {}: {}",
                     &out_file_path.to_str().unwrap(),
                     e);
            continue;
        }
    }

}

fn decode(matches: &ArgMatches, out: &Path) {
    let filenames: Vec<&str> = match matches.values_of("files") {
        Some(f) => f.collect(),
        None => {
            println!("Please provide a list of files");
            exit(1);
        }
    };

    for filename in filenames {
        let in_file = match File::open(PathBuf::from(filename)) {
            Ok(f) => f,
            Err(e) => {
                println!("Unable to open {}: {}", &filename, e);
                continue;
            }
        };

        let lit = match Lightmap::from_file(in_file) {
            Ok(l) => l,
            Err(e) => {
                println!("Error reading LIT file {}: {}", &filename, e);
                continue;
            }
        };

        let mut out_file_path = PathBuf::from(out);
        out_file_path.push(filename);
        out_file_path.set_extension("json");

        let out_file_parent = out_file_path.parent().unwrap();
        if !out_file_parent.exists() {
            if let Err(e) = create_dir_all(out_file_parent) {
                println!("Error creating output directory: {}", e);
                continue;
            };
        }

        let out_file = match File::create(&out_file_path) {
            Ok(f) => f,
            Err(e) => {
                println!("Unable to open {}: {}", &out_file_path.to_str().unwrap(), e);
                continue;
            }
        };

        let mut writer = BufWriter::new(out_file);
        if let Err(e) = serde_json::to_writer_pretty(&mut writer, &lit) {
            println!("Error writing file {}: {}",
                     &out_file_path.to_str().unwrap(),
                     e);
            continue;
        }
    }
}
fn main() {
    let yaml = load_yaml!("litter.yaml");
    let matches = clap::App::from_yaml(yaml).get_matches();

    let out_dir_str = matches.value_of("out_dir").unwrap();
    let out_dir = Path::new(out_dir_str);

    match matches.subcommand() {
        ("encode", Some(sub)) => {
            encode(sub, &out_dir);
        }
        ("decode", Some(sub)) => {
            decode(sub, &out_dir);
        }
        _ => {
            println!("Please use a valid subcommand. Try `./litter --help` for more information.");
            exit(1);
        }
    }
}
