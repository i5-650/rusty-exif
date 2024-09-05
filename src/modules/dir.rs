use std::path::PathBuf;
use rayon::prelude::*;
use anyhow::{Result, Error};

use crate::models::Image;
use crate::core;
use crate::utils;

pub fn dir_module(folder_name: &String, split: bool, export_file: &Option<String>) -> Result<(), Error> {
    let folder = PathBuf::from(folder_name);
    let mut exifs = core::from_dir(folder);

    if split {

        let list_err = exifs.as_parallel_slice_mut()
            .par_iter_mut()
            .map(|img: &mut Image| {
                let serialized = serde_json::to_string_pretty(&img).expect("Must be a Map");
                utils::write_json_to_file(serialized, &mut img.name)
            })
            .collect::<Vec<Result<()>>>();

        if !list_err.is_empty() {
            println!("[/!\\] Error encountered while creating/writing to files.");
        }

    } else if let Some(mut export) = export_file.to_owned() {
        let serialized = serde_json::to_string_pretty(&exifs).expect("Map must be <String, String>");
        utils::write_json_to_file(serialized, &mut export)?;
    }

    Ok(())
}

