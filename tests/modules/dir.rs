use tempdir::TempDir;
use anyhow::{Error, Result};
use std::fs;

extern crate rsexif;

use rsexif::modules;


#[test]
fn dir_module_no_split() -> Result<(), Error>{
    // Given
    let tempdir = TempDir::new("dir_module_split")?;

    let export_file = tempdir.path().join("export_test_file.json");


    // Act
    let res = modules::dir_module(&"./tests/resources/".to_string(), false, &Some(export_file.display().to_string()));

    // Assert
    assert!(res.is_ok());
    assert!(export_file.as_path().exists());
    assert_ne!(0, std::fs::metadata(export_file)?.len());

    Ok(())
}

#[test]
fn dir_module_split() -> Result<(), Error>{
    // Act
    let res = modules::dir_module(&"./tests/resources/".to_string(), true, &None);

    // Assert
    assert!(res.is_ok());

    let paths = fs::read_dir("./tests/resources/")?;

    let mut count = 0;
    for path in paths {
        let path = path?.path();
        if path.extension().unwrap() == "json" {
            assert_ne!(0, std::fs::metadata(path.clone())?.len());
            count+=1;
            std::fs::remove_file(path)?;
        }
    }
    assert_eq!(3, count);

    Ok(())
}
