use clap::{Parser, Subcommand};
use rayon::{
    prelude::ParallelSliceMut,
    iter::{
        IntoParallelRefMutIterator,
        ParallelIterator
    }
};
use exifs::structures::Image;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::println;
use anyhow::{Result, anyhow};

pub mod exifs;

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
    },

    #[command(arg_required_else_help = true, long_flag = "dir", short_flag = 'd', about = "Extract exif from every files in a directory")]
    Dir {
        #[arg(value_name = "folder", required = true, help = "directory containing images to extract exifs from")]
        folder: String,
        
        #[arg(value_name = "split", required = false, conflicts_with = "export_folder", short = 's', long = "split", help = "Wether you decide to store all exifs into one file or multiples")]
        split: Option<bool>,

        #[arg(value_name = "export", required_unless_present = "split", short = 'e', long = "export", help = "The name of the Json file containing all the exifs")]
        export_folder: Option<String>,
    }
}


fn main() -> Result<()> {
    let args = Args::parse();

    let status = match &args.command {
        Commands::File { file, export } => {
            let m_file = file.as_str();
            let export = export.to_owned();
            file_module(m_file.to_string(), export)
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


fn file_module(filename: String, export_file: Option<String>) -> Result<(), io::Error> {
    let exifs = exifs::from_file(filename);

    if export_file.is_some() {
        let serialized = serde_json::to_string_pretty(&exifs).expect("Map must be <String, String>");
        let mut export = export_file.unwrap();
        let mut json_file = create_json_file(&mut export)?;
        json_file.write_all(serialized.as_bytes())?
    } else {
        for (key, value) in exifs {
			println!("{}: {}", key, value);
		}
    }
    Ok(())
}

fn create_json_file(filename: &mut String) -> Result<File, io::Error> {
    if !filename.ends_with(".json") {
        filename.push_str(".json");
    }
    File::create(filename)
}


fn folder_module(folder_name: &String, split: Option<bool>, export_file: &Option<String>) -> Result<(), io::Error> {
    let folder = PathBuf::from(folder_name);
    let mut exifs = exifs::from_folder(folder);

    if split.is_some() && split.unwrap() {
        let list_err = exifs.as_parallel_slice_mut()
            .par_iter_mut()
            .map(|img: &mut Image| {
                let serialized = serde_json::to_string_pretty(&img).expect("Map must be <String, String>");
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

