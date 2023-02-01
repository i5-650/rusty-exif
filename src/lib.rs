use std::path::PathBuf;
use std::{fs, collections::HashMap, collections::BTreeMap};
use scanf::sscanf;
use rayon::prelude::*;

use models::image::Image;

pub mod models;

const GOOGLE_MAP: &str = "googleMap";


pub fn from_file(path: String) -> HashMap<String, BTreeMap<String,String>> {

    let metadata = match rexiv2::Metadata::new_from_path(path) {
        Ok(m) => m,
        Err(e) => {
            println!("[*] Error while reading the exif from a file: {}", e);
            return HashMap::new();
        }
    };

    let tags = match metadata.get_exif_tags() {
        Ok(t) => t,
        Err(e) => {
            println!("[*] Error while retreving the exif: {}", e);
            return HashMap::new();
        }
    };

    let map_data = tags.iter()
        .map(|tag| {
            let value = match metadata.get_tag_interpreted_string(tag) {
                Ok(val) => val,
                Err(_) => String::from("Failed to convert to string"),
            };

            // Exifs tags are like: Exif.Categ.TheTag
            let parts: Vec<&str> = tag.split('.').collect();
            if parts.len() >= 3 {
                let category = parts[1].to_string();
                let tag_name = parts[2..].join(".");                
                (category, tag_name, value)
            } else {
                let category = "Unknown".to_string();
                let tag_name = parts[parts.len() -1].to_string();
                (category, tag_name, value)
            }
        })
        // We want the exifs to be in categories so we make a map of map
        .fold(HashMap::new(), |mut acc: HashMap<String, BTreeMap<String, String>>, (category, tag_name, value)| {
            // Use a BTreeMap to keep the elements sorted (better readability)
            acc.entry(category)
                .or_default()
                .insert(tag_name, value);
            acc
        });

	add_google_map(map_data)
}



fn add_google_map(mut map_data :HashMap<String, BTreeMap<String, String>>) -> HashMap<String, BTreeMap<String, String>> {

    if !map_data.contains_key("GPSInfo") {
        return map_data;
    } 

    let gps_info = map_data.get_mut("GPSInfo").expect("Impossible missing GPSInfo");

    if let (Some(longitude), Some(latitude)) = (gps_info.get("GPSLatitude"), gps_info.get("GPSLongitude")) {
        gps_info.insert(
            GOOGLE_MAP.to_string(), 
            format!("https://www.google.com/maps/search/?api=1&query={},{}",
                to_decimal(latitude), 
                to_decimal(longitude)
            )
        );
    }
    map_data
}

fn to_decimal(dms: &str) -> f64 {
    let mut degrees: f64 = 0.0;
    let mut minutes: f64 = 0.0;
    let mut seconds: f64 = 0.0;

	if sscanf!(dms, "{f64} deg {f64}' {f64}\"", degrees, minutes, seconds).is_err() {
        return 0.0;
    }

	degrees + minutes / 60.0 + seconds / 3600.0
}

pub fn from_folder(_path: PathBuf) -> Vec<Image> {
	let files = fs::read_dir(_path).expect("Couldn't read the directory given");

	files.par_bridge()
		.filter_map(|f| f.ok())
		.filter(|f| !f.path().ends_with(".DS_Store") && !f.path().ends_with("/"))
		.map(|f| {
			let entry_path = f.path().display().to_string();
			Image{
				name: entry_path.clone(),
				exifs: from_file(entry_path)
			}
		}).collect::<Vec<Image>>()
}
