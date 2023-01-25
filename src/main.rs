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
		.args(["file", "folder"])
))]
struct Args {
	#[arg(short, long)]
	export: Option<String>,
	file: Option<PathBuf>,
	#[arg(short = 'f', long)]
	folder: Option<String>
}

fn main() {
	let args = Args::parse();

	if args.file.is_some() && args.export.is_some() {
		let predic = check_extension(&mut args.export.unwrap());
		let mut output_file = predic.unwrap();
		let data_to_jsonify = exif_mapper::map_exif_from_file(PathBuf::from(args.file.unwrap()));
		let output = serde_json::to_string_pretty(&data_to_jsonify);
		if !output_file.write(output.unwrap().as_bytes()).is_ok() {
			println!("Couln't save file...");
		}
	}
	else if args.file.is_some() {
		for (key, value) in exif_mapper::map_exif_from_file( PathBuf::from(args.file.unwrap())) {
			println!("{}: {}", key, value);
		}
	}
	else if args.export.is_some() && args.folder.is_some() {
		let predic = check_extension(&mut args.export.unwrap());

		let mut output_file = predic.unwrap();
		let data_to_jsonify = exif_mapper::map_exif_from_folder(PathBuf::from(args.folder.unwrap()));
		let output = serde_json::to_string_pretty(&data_to_jsonify);
		if !output_file.write(output.unwrap().as_bytes()).is_ok() {
			println!("Couln't save file...");
		}
		
	}
	else {
		println!("No file or folder specified, use -h for help");
	}
}

fn check_extension(filename: &mut String) -> Result<File, std::io::Error> {
	if !filename.ends_with(".json") {
		filename.push_str(".json");
	}
	return File::create(filename);
}

