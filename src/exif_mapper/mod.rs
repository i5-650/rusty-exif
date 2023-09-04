use std::path::PathBuf;
use std::{fs, collections::HashMap};
use sscanf::scanf;
use rayon::prelude::*;

use self::data_structures::Image;

pub mod data_structures;

const GOOGLE_MAP: &str = "googleMap";

pub fn map_exif_from_file(_path_to_image: PathBuf) -> HashMap<String, String> {
	let mut map_data = HashMap::new();

	let file = std::fs::File::open(_path_to_image.as_path()).expect("Couldn't open the image");
	let mut bufreader = std::io::BufReader::new(&file);
	let exifreader = exif::Reader::new();

	if let Ok(exif) = exifreader.read_from_container(&mut bufreader){

		exif.fields()
		.map(|f| (f.tag.to_string(), f.display_value().to_string()))
		.for_each(|f| {
			let (key, value) = f;
			map_data.insert(key, format_value(value));

		});
	}
	return add_google_map(map_data);
}

fn add_google_map(mut map_data :HashMap<String, String>) -> HashMap<String, String> {
	if map_data.contains_key("GPSLatitude") && map_data.contains_key("GPSLongitude") {
		map_data.insert(
			GOOGLE_MAP.to_string(), 
			format!("https://www.google.com/maps/search/?api=1&query={},{}",
					convert_dms_to_decimal(map_data.get("GPSLatitude").unwrap()), 
					convert_dms_to_decimal(map_data.get("GPSLongitude").unwrap())
				));
	}
	return map_data;
}

fn format_value(input: String) -> String {
	return input.replace("\"", "").to_string();
}

fn convert_dms_to_decimal(dms: &String) -> f64 {
	let parsed = scanf!(dms, "{} deg {} min {} sec", f64, f64, f64).unwrap();
	let (degrees, minutes, seconds) = parsed;
	return degrees + minutes / 60.0 + seconds / 3600.0;
}

pub fn map_exif_from_folder(_path: PathBuf) -> Vec<Image> {
	let files = fs::read_dir(_path).expect("Couldn't read the directory given");

	let res = files.par_bridge()
	.filter_map(|f| {f.ok()})
	.filter(|f| !f.path().ends_with(".DS_Store") && !f.path().ends_with("/"))
	.map(|f| {
		let entry_path = f.path();
		return data_structures::Image{
			name: entry_path.display().to_string(),
			exifs: map_exif_from_file(entry_path)
		};
	}).collect::<Vec<Image>>();

	return res;
}