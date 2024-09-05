use scanf::sscanf;
use std::collections::{HashMap, BTreeMap};
use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::Write;

pub const GOOGLE_MAP: &str = "googleMap";

pub fn to_decimal(dms: &str) -> f64 {
    let mut degrees: f64 = 0.0;
    let mut minutes: f64 = 0.0;
    let mut seconds: f64 = 0.0;

	if sscanf!(dms, "{f64} deg {f64}' {f64}\"", degrees, minutes, seconds).is_err() {
        return 0.0;
    }

	degrees + minutes / 60.0 + seconds / 3600.0
}

pub fn add_google_map(mut map_data :HashMap<String, BTreeMap<String, String>>) -> HashMap<String, BTreeMap<String, String>> {

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

#[inline(always)]
pub fn create_json_file(filename: &mut String) -> Result<File, std::io::Error> {
    if !filename.ends_with(".json") {
        filename.push_str(".json");
    }
    File::create(filename)
}

pub fn write_json_to_file(content: String, path: &mut String) -> Result<()> {
    let mut json_file = create_json_file(path)?;
    match json_file.write_all(content.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!(e))
    }
}

