use anyhow::{Error, Result};

use crate::core;
use crate::utils;

pub fn file_module(filename: String, export_file: Option<String>, json: bool) -> Result<(), Error> {
    let exifs = core::from_file(filename);

    if export_file.is_some() || json {
        let serialized = serde_json::to_string_pretty(&exifs).expect("Map must be <String, String>");

        if json {
            println!("{}", serialized);
        } else {
            let mut export = export_file.unwrap();
            utils::write_json_to_file(serialized, &mut export)?
        }

    } else {
        exifs.iter().for_each(|(categ, sub_exifs)| {
            println!("{}", categ);
            sub_exifs.iter().for_each(|(key, val)| {
                println!("    {}: {}", key, val);
            })
        });
    }
    Ok(())
}

