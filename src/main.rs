/**
 * Licence: MIT
 * Author: loic-prn
 * Date: 2022-11-19
 *
 */

use std::any::Any;
use std::collections::HashMap;
use clap::{ArgGroup, Parser};
use std::fs::File;
use std::println;
use rayon::prelude::{ParallelSliceMut, IntoParallelRefMutIterator, ParallelIterator};
use std::io::Write;
use std::path::PathBuf;
use exif_mapper::data_structures::Image;

pub mod exif_mapper;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(group(
	ArgGroup::new("incompatibles")
		.required(false)
		.args(["file", "folder"])
))]
struct Args {
	#[arg(short = 'e', long = "export")]
	export: Option<String>,
	file: Option<PathBuf>,
	#[arg(short = 'f', long = "folder")]
	folder: Option<String>,
	#[arg(short = 's', long = "split")]
	separate: bool,
}

impl Args {
	pub fn export_and_file(&self) -> bool{
		return self.export.is_some() && self.file.is_some();
	}

	pub fn file_no_export(&self) -> bool {
		return self.file.is_some() && self.export.is_none();
	}

	pub fn folder_no_split(&self) -> bool {
		return self.folder.is_some() && !self.separate;
	}

	pub fn folder_split(&self) -> bool {
		return self.folder.is_some() && self.separate;
	}
}

fn main() {
	let args = &Args::parse();

	if args.export.is_some() || args.folder_no_split() {
		let filename = args.export.as_deref().expect("Couldn't get the export filename");
		let mut output_file = check_extension(&mut filename.to_string());
		
		let to_write :Box<dyn Any>;

		if args.export_and_file() {
			let file_par = args.file.as_deref().expect("Invalid file given as parameter");
			let image_path = PathBuf::from(file_par);
			to_write = Box::new(exif_mapper::map_exif_from_file(image_path));

		} else {
			let folder_par = args.folder.as_deref().expect("Couldn't get the input folder");
			let folder_path = PathBuf::from(folder_par);
			to_write = Box::new(exif_mapper::map_exif_from_folder(folder_path));
		};

		if let Some(map) = to_write.downcast_ref::<HashMap<String, String>>() {
			let serialized = serde_json::to_string_pretty(map).expect("Couldn't serialize data");
			output_file.write(serialized.as_bytes()).expect("Couldn't write data in the file.");
		} else if let Some(list_img) = to_write.downcast_ref::<Vec<Image>>() {
			let serialized = serde_json::to_string_pretty(list_img).expect("Couldn't serialize data");
			output_file.write(serialized.as_bytes()).expect("Couldn't write data in the file.");
		}

		return;
	}

	if args.file_no_export() {
		let file_par = args.file.as_deref().expect("Invalid file given as parameter");
		let image_path = PathBuf::from(file_par);
		let to_print= exif_mapper::map_exif_from_file(image_path);
		for (key, value) in to_print {
			println!("{}: {}", key, value);
		}
		return;
	}

	if args.folder_split() {
		let folder_par = args.folder.as_deref().expect("Couldn't get the input folder");
		let folder_path = PathBuf::from(folder_par);
		let mut to_write = exif_mapper::map_exif_from_folder(folder_path);
		to_write.as_parallel_slice_mut()
			.par_iter_mut()
			.for_each(|img| {
				let serialized = serde_json::to_string_pretty(&img).expect("Couldn't serialize data");
				img.name.push_str(".json");
				let mut output_file = File::create(&img.name).expect("Coulnd't create an output file named as one of the input files");
				output_file.write(serialized.as_bytes()).expect("Couldn't write data in the file.");
			});
		return;
	}

	println!("Something went wrong with parameters");
}

fn check_extension(filename: &mut String) -> File {
	if !filename.ends_with(".json") {
		filename.push_str(".json");
	}
	return File::create(filename).expect("Couldn't create the ouput file.");
}
