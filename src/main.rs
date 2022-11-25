/**
 * Licence: MIT
 * Author: loic-prn
 * Date: 2022-11-19
 * 
 */

use std::io::Write;
use std::path::PathBuf;
use std::fs::File;
use clap::{ArgGroup, Parser};

pub mod exif_mapper;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(group(
    ArgGroup::new("incompatibles")
        .required(false)
        .args(["file", "folder"]),
))]
struct Args {
    #[arg(short, long)]
    export: Option<String>,
    #[arg(short= 'f', long)]
    file: Option<String>,
    #[arg(short = 'F', long)]
    folder: Option<String>
}

fn main() {
    let args = Args::parse();
    
    if args.file.is_some() && args.export.is_some() {
        let mut filename = args.export.unwrap();
        if !filename.ends_with(".json") {
            filename.push_str(".json");
        }
        let predic = File::create(filename);

        if predic.is_ok() {
            let mut output_file = predic.unwrap();
            let data_to_jsonify = exif_mapper::map_exif(PathBuf::from(args.file.unwrap()));
            let output = serde_json::to_string_pretty(&data_to_jsonify);
            if !output_file.write(output.unwrap().as_bytes()).is_ok() {
                println!("Couln't save file...");
            }
        }
    }
    else if args.file.is_some() {
        for (key, value) in exif_mapper::map_exif( PathBuf::from(args.file.unwrap())) {
            println!("{}: {}", key, value);
        }
    }
    else if args.export.is_some() && args.folder.is_some() {
        let mut filename = args.export.unwrap();
        if !filename.ends_with(".json") {
            filename.push_str(".json");
        }
        let predic = File::create(filename);

        if predic.is_ok() {
            let mut output_file = predic.unwrap();
            let data_to_jsonify = exif_mapper::json_string_from_dir(PathBuf::from(args.folder.unwrap()));
            let output = serde_json::to_string_pretty(&data_to_jsonify);
            if !output_file.write(output.unwrap().as_bytes()).is_ok() {
                println!("Couln't save file...");
            }
        }
    }
}


