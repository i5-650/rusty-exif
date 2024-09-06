use anyhow::{Result, Error};
use tempdir::TempDir;

extern crate rsexif;

use rsexif::modules;

#[test]
fn file_module_export() -> Result<(), Error>{
    // Given
    let tempdir = TempDir::new("file_module_export")?;

    let export_file = tempdir.path().join("export_test_file.json");


    // Act
    let res = modules::file_module("./tests/resources/Canon_40D.jpg".to_string(), Some(export_file.display().to_string()), false);

    // Assert
    assert!(res.is_ok());
    assert!(export_file.as_path().exists());

    Ok(())
}

#[test]
fn file_module_print_json() {
    // Act
    let res = modules::file_module("./tests/resources/Canon_40D.jpg".to_string(), None, true);

    // Assert
    assert!(res.is_ok());
}

#[test]
fn file_module_print() {
    // Act
    let res = modules::file_module("./tests/resources/Canon_40D.jpg".to_string(), None, false);

    // Assert
    assert!(res.is_ok());
}
