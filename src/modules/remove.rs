use anyhow::{Error, Result};
use std::fs::{metadata, read_dir};
use rayon::prelude::*;

pub fn remove_module(path: &String) -> Result<(), Error> {
    if metadata(path)?.is_dir() {
        let paths = read_dir(path)?;
        let _ = paths
            .par_bridge()  // Parallelize the read_dir entries
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    e.path().to_str().map(|s| s.to_string())  // Convert path to String
                })
                .map(|item| {
                        if let Err(e) = remove_exifs(&item) {
                            println!("[*] Error removing exifs from {}: {}", item, e);
                        }
                    })  // Apply remove_exifs if path is valid
        }).collect::<Vec<_>>();

        Ok(())

    } else {
        remove_exifs(path)
    }
}

fn remove_exifs(path: &String) -> Result<(), Error> {
    let meta = rexiv2::Metadata::new_from_path(path.clone())?;
    meta.clear();
    meta.save_to_file(path)?;
    Ok(())
}

