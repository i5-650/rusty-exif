use clap::{Parser, Subcommand};
use rayon::{
    prelude::ParallelSliceMut,
    iter::{
        IntoParallelRefMutIterator,
        ParallelIterator
    }
};

extern crate rsexif;

use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::println;
use anyhow::{Result, anyhow, Error};


#[derive(Parser)]
#[command(
    author="i5-650", 
    about="Exif extraction tools", 
    long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
enum Commands {

    #[command(arg_required_else_help = true, long_flag = "file", short_flag = 'f', about = "Extract exif from a single file")]
    File {
        #[arg(value_name = "file", required = true, help = "image to extract exif from")]
        file: String,
        
        #[arg(value_name = "export", required = false, short = 'e', long = "export", help = "Json file to output exifs to")]
        export: Option<String>,

        #[arg(value_name = "json", required = false, conflicts_with = "export", short = 'j', long = "json", help = "Print the result as JSON")]
        json: bool,
    },

    #[command(arg_required_else_help = true, long_flag = "dir", short_flag = 'd', about = "Extract exif from every files in a directory")]
    Dir {
        #[arg(value_name = "folder", required = true, help = "directory containing images to extract exifs from")]
        folder: String,
        
        #[arg(value_name = "split", required = false, conflicts_with = "export_folder", short = 's', long = "split", help = "Wether you decide to store all exifs into one file or multiples")]
        split: bool,

        #[arg(value_name = "export", required_unless_present = "split", short = 'e', long = "export", help = "The name of the Json file containing all the exifs")]
        export_folder: Option<String>,
    }
}


fn main() -> Result<(), Error> {
    let args = Args::parse();

    let status = match &args.command {
        Commands::File { file, export, json } => {
            let m_file = file.as_str();
            let export = export.to_owned();
            let json = json.to_owned();
            file_module(m_file.to_string(), export, json)
        }, 

        Commands::Dir { folder, split, export_folder } => {
            folder_module(folder, *split, export_folder)
        }
    };

    if let Err(e) = status {
        println!("Error while extracing exifs");
        Err(anyhow!(e))
    } else {
        Ok(())
    }
}


fn file_module(filename: String, export_file: Option<String>, json: bool) -> Result<(), Error> {
    let exifs = rsexif::from_file(filename);

    if export_file.is_some() || json {
        let serialized = serde_json::to_string_pretty(&exifs).expect("Map must be <String, String>");

        if json {
            println!("{}", serialized);
        } else {
            let mut export = export_file.unwrap();
            let mut json_file = create_json_file(&mut export)?;
            json_file.write_all(serialized.as_bytes())?
        }

    } else {
        exifs.iter().for_each(|(categ, sub_exifs)| {
            println!("{}", categ);
            sub_exifs.iter().for_each(|(key, val)| {
                println!("\t{}: {}", key, val);
            })
        });
    }
    Ok(())
}

#[inline(always)]
fn create_json_file(filename: &mut String) -> Result<File, io::Error> {
    if !filename.ends_with(".json") {
        filename.push_str(".json");
    }
    File::create(filename)
}


fn folder_module(folder_name: &String, split: bool, export_file: &Option<String>) -> Result<(), Error> {
    let folder = PathBuf::from(folder_name);
    let mut exifs = rsexif::from_folder(folder);

    if split {

        let list_err = exifs.as_parallel_slice_mut()
            .par_iter_mut()
            .map(|img: &mut rsexif::models::Image| {
                let serialized = serde_json::to_string_pretty(&img).expect("Must be a Map");
                img.name.push_str(".json");
                exif_to_json(serialized, &img.name)
            })
            .collect::<Vec<Result<()>>>();

        if !list_err.is_empty() {
            println!("[/!\\] Error encountered while creating/writing to files.");
        }

    } else if let Some(mut export) = export_file.to_owned() {
        let mut export = create_json_file(&mut export)?;
        let serialized = serde_json::to_string_pretty(&exifs).expect("Map must be <String, String>");
        export.write_all(serialized.as_bytes())?
    }

    Ok(())
}

fn exif_to_json(content: String, path: &String) -> Result<()> {
    let mut json_file = File::create(path)?;
    match json_file.write_all(content.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!(e))
    }
}

