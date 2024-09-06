
extern crate rsexif;

use rsexif::core;
use std::path::PathBuf;

#[test]
fn from_file_test() {
    // Act
    let exifs = core::from_file("./tests/resources/Canon_40D.jpg".to_string());

    
    // Assert
    assert!(!exifs.is_empty());
    assert_eq!(5, exifs.len());
    assert!(exifs.contains_key("Thumbnail"));
    assert!(exifs.contains_key("GPSInfo"));
    assert!(exifs.contains_key("Photo"));
    assert!(exifs.contains_key("Image"));
    assert!(exifs.contains_key("Iop"));
}


#[test]
fn from_dir_test() {
    // Given
    let folder_pathbuf = PathBuf::from("./tests/resources/");

    // Act
    let exifs = core::from_dir(folder_pathbuf);


    // Assert
    assert!(!exifs.is_empty());
    assert_eq!(3, exifs.len());

    let mut checked = 0;
    exifs.iter().for_each(|e| {
        let name = e.name.as_str();
        if name == "./tests/resources/Canon_40D.jpg" {
            checked+=1;
            assert!(e.exifs.contains_key("Thumbnail"));
            assert!(e.exifs.contains_key("GPSInfo"));
            assert!(e.exifs.contains_key("Photo"));
            assert!(e.exifs.contains_key("Image"));
            assert!(e.exifs.contains_key("Iop"));
        }

        if name == "./tests/resources/Canon_40D_photoshop_import.jpg" {
            checked+=1;
            assert!(e.exifs.contains_key("Photo"));
            assert!(e.exifs.contains_key("Image"));
            assert!(e.exifs.contains_key("Thumbnail"));
        }


        if name == "./tests/resources/Nikon_D70.jpg" {
            checked+=1;
            assert!(e.exifs.contains_key("Photo"));
            assert!(e.exifs.contains_key("Image"));
            assert!(e.exifs.contains_key("Thumbnail"));
        }
    });

    assert_eq!(3, checked);
}
