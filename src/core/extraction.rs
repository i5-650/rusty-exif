use crate::utils;
use std::collections::{HashMap, BTreeMap};
use crate::models::Image;
use std::fs;
use std::path::PathBuf;
use rayon::prelude::*;


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

	utils::add_google_map(map_data)
}

pub fn from_dir(_path: PathBuf) -> Vec<Image> {
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


