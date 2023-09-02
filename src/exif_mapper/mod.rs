use std::path::PathBuf;
use std::{fs, collections::HashMap};
use sscanf::scanf;

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
	let files_list = list_files_in_dir(_path);
	let mut data_out = vec![];

	files_list.iter().for_each(|file|{
		data_out.push(data_structures::Image {
			name: file.as_path().display().to_string(),
			exifs: map_exif_from_file(file.to_owned())
		});
	});
	return data_out;
}


fn list_files_in_dir(path: PathBuf) ->  Vec<PathBuf>{
	let mut file_list = vec![];

	let files = fs::read_dir(path).expect("Couldn't read the directory given");

	for file in files {
		let path_file = file.as_ref()
			.expect("Couldn't get the ref on a file in the directory")
			.path();

		if !path_file.ends_with(".DS_Store") && !path_file.ends_with("/") {
			file_list.push(path_file);
		}
	}

	return file_list;
}
