use clap::{Parser, Subcommand};
use anyhow::{Result, Error};

extern crate rsexif;
use rsexif::modules;

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
        #[arg(value_name = "dir", required = true, help = "directory containing images to extract exifs from")]
        dir: String,
        
        #[arg(value_name = "split", required = false, conflicts_with = "export_folder", short = 's', long = "split", help = "Wether you decide to store all exifs into one file or multiples")]
        split: bool,

        #[arg(value_name = "export", required_unless_present = "split", short = 'e', long = "export", help = "The name of the Json file containing all the exifs")]
        export_folder: Option<String>,
    },

    #[command(arg_required_else_help = true, long_flag = "remove", short_flag = 'r', about = "Remove exifs")]
    Rm {
        #[arg(value_name = "path", required = true, help = "file to remove exifs from")]
        path: String
    }
}


fn main() -> Result<(), Error> {
    let args = Args::parse();

   match &args.command {
        Commands::File { file, export, json } => {
            let m_file = file.as_str();
            let export = export.to_owned();
            let json = json.to_owned();
            modules::file_module(m_file.to_string(), export, json)
        }, 

        Commands::Dir { dir, split, export_folder } => {
            modules::dir_module(dir, *split, export_folder)
        },

        Commands::Rm { path } => {
            modules::remove_module(path)
        }
    }
}
