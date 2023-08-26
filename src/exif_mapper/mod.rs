use std::path::PathBuf;
use std::{fs, collections::HashMap};
use sscanf::scanf;

use self::data_structures::Image;

mod data_structures;

pub fn map_exif_from_file(_path_to_image: PathBuf) -> HashMap<String, String> {
	let mut map_data = HashMap::new();

	if let Ok(file) = std::fs::File::open(_path_to_image.as_path()) {
		let mut bufreader = std::io::BufReader::new(&file);
		let exifreader = exif::Reader::new();

		if let Ok(exif) = exifreader.read_from_container(&mut bufreader) {

			exif.fields()
				.map(|f| (f.display_value().to_string(), f.tag.to_string()))
				.for_each(|f| {
					let mut value = f.0;
					if value.starts_with("\"") && value.ends_with("\"") {
						value = value.strip_prefix("\"").unwrap().strip_suffix("\"").unwrap().to_string();
					}
					map_data.insert(f.1, value);
				});

			if map_data.contains_key("GPSLatitude") && map_data.contains_key("GPSLongitude") {
				map_data.insert(
					"googleMap".to_string(), 
					format!("https://www.google.com/maps/search/?api=1&query={},{}",
						 convert_dms_to_decimal(map_data.get("GPSLatitude").unwrap()), 
						 convert_dms_to_decimal(map_data.get("GPSLongitude").unwrap())
						)
				);
			}
		}
	}
	return map_data;
}

pub fn convert_dms_to_decimal(dms: &String) -> f64 {
	let parsed = scanf!(dms, "{} deg {} min {} sec", f64, f64, f64).unwrap();
	let degrees = parsed.0;
	let minutes = parsed.1;
	let seconds = parsed.2;
	return degrees + minutes / 60.0 + seconds / 3600.0;
}

pub fn map_exif_from_folder(_path: PathBuf) -> Vec<Image> {
	let files_list = get_file_list(_path);
	let mut data_out = vec![];

	for pathbuf_file in files_list {
		data_out.push(data_structures::Image { 
			name: pathbuf_file.as_path().display().to_string(), 
			exifs: map_exif_from_file(pathbuf_file) }
		);
	}
	return data_out;
}


fn get_file_list(_path: PathBuf) ->  Vec<PathBuf>{
	let mut file_list = vec![];

	for file in fs::read_dir(_path).unwrap() {
		if !file.as_ref().unwrap().path().ends_with(".DS_Store") && !file.as_ref().unwrap().path().ends_with("/") {
			file_list.push(
				file.unwrap().path()
			);
		}
	}

	return file_list;
}
