use std::collections::{HashMap, BTreeMap};
use tempdir::TempDir;
use regex::Regex;

extern crate rsexif;

use rsexif::utils;


#[test]
fn to_decimal_test_ok() {
    // Given
    let dms = "50 deg 28' 36.63\"";
    
    // Act
    let decimal = utils::to_decimal(dms);

    // Assert
    assert_ne!(0.0, decimal);
}


#[test]
fn to_decimal_test_ko() {
    // Given
    let dms = "someRandomValue";
    
    // Act
    let decimal = utils::to_decimal(dms);

    // Assert
    assert_eq!(0.0, decimal);
}


#[test]
fn add_google_map_existing() {
    // Given
    let mut map = HashMap::new();
    
    let mut sub_map = BTreeMap::new();
    sub_map.insert("GPSLatitude".to_string(), "48 deg 51' 52.9776\"".to_string());
    sub_map.insert("GPSLongitude".to_string(), "2 deg 20' 56.4504\"".to_string());
    map.insert("GPSInfo".to_string(), sub_map);


    // Act
    let result_map = utils::add_google_map(map);

    // Assert
    assert!(!result_map.is_empty());
    assert!(result_map.contains_key("GPSInfo"));
    assert!(result_map.get("GPSInfo").unwrap().contains_key(utils::GOOGLE_MAP));
    
    // Assert
    let re = Regex::new(r"^https://www\.google\.com/maps/search/\?api=1&query=(-?\d+\.\d+),(-?\d+\.\d+)$").unwrap(); 
    let google_map = result_map.get("GPSInfo").unwrap().get(utils::GOOGLE_MAP).unwrap();
    assert!(re.is_match(google_map));
}


#[test]
fn add_google_map_nothing() {
    // Given
    let map: HashMap<String, BTreeMap<String, String>> = HashMap::new();

    // Act
    let result_map = utils::add_google_map(map);

    // Assert
    assert!(result_map.is_empty());
}

#[test]
fn add_google_map_not_enough() {
    // Given
    let mut map = HashMap::new();
    map.insert("GPSInfo".to_string(), BTreeMap::new());

    // Act
    let result_map = utils::add_google_map(map);


    // Assert
    assert!(!result_map.is_empty());
    assert!(result_map.contains_key("GPSInfo"));
    assert!(!result_map.get("GPSInfo").unwrap().contains_key(utils::GOOGLE_MAP));
}


#[test]
fn create_json_file_test() -> Result<(), anyhow::Error> {
    // Given
    let tmp_dir = TempDir::new("create_json_file_test")?;
    let tmp_file_path = tmp_dir.path().join("tmp_json_file");

    // Act
    let res = utils::create_json_file(&mut tmp_file_path.display().to_string());

    // Assert
    assert!(res.is_ok());
    tmp_dir.close()?;
    Ok(())
}


#[test]
fn write_json_to_file_test() -> Result<(), anyhow::Error> {
    // Given
    let tmp_dir = TempDir::new("write_json_to_file_test")?;
    let tmp_file_path = tmp_dir.path().join("tmp_json_file");

    // Act
    let res = utils::write_json_to_file("{\"example\":\"test\"}".to_string(), &mut tmp_file_path.display().to_string());

    // Assert
    assert!(res.is_ok());
    Ok(())
}
