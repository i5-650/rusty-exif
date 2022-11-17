use std::{fs, collections::HashMap};
use std::io::Write;
use std::path::PathBuf;
use std::fs::File;
use serde::{Serialize, Deserialize};
use clap::{ArgGroup, Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(group(
    ArgGroup::new("incompatibles")
        .required(false)
        .args(["file", "folder"]),
))]
struct Args {
    #[arg(short, long)]
    export: Option<String>,
    #[arg(short= 'f', long)]
    file: Option<String>,
    #[arg(short = 'F', long)]
    folder: Option<String>
}

#[derive(Serialize, Deserialize)]
struct Image {
    image_name: String,
    #[serde(flatten)]
    exif_fields: HashMap<String, String>
}

#[derive(Serialize, Deserialize)]
struct Data {
    images: Vec<Image>
}

fn main() {
    let args = Args::parse();
    
    if args.file.is_some() && args.export.is_some() {
        let predic = File::create(args.export.unwrap());

        if predic.is_ok() {
            let mut output_file = predic.unwrap();
            let data_to_jsonify = map_exif(PathBuf::from(args.file.unwrap()));
            let output = serde_json::to_string_pretty(&data_to_jsonify);
            if !output_file.write(output.unwrap().as_bytes()).is_ok() {
                println!("Couln't save file...");
            }
        }
    }
    else if args.file.is_some() {
        for (key, value) in map_exif( PathBuf::from(args.file.unwrap())) {
            println!("{}: {}", key, value);
        }
    }
    else if args.export.is_some() && args.folder.is_some() {
        let predic = File::create(Args::parse().export.unwrap());

        if predic.is_ok() {
            let mut output_file = predic.unwrap();
            let data_to_jsonify = json_string_from_dir(PathBuf::from(args.folder.unwrap()));
            let output = serde_json::to_string_pretty(&data_to_jsonify);
            if !output_file.write(output.unwrap().as_bytes()).is_ok() {
                println!("Couln't save file...");
            }
        }
    }
}


fn json_string_from_dir(_path: PathBuf) -> Data {
    let files_list = get_file_list(_path);
    let mut data_out = Data { images: vec![] };

    for pathbuf_file in files_list {
        data_out.images.push(Image {
            image_name: pathbuf_file.as_path().display().to_string(), 
            exif_fields: map_exif(pathbuf_file)});
    }
    return data_out;
}

fn map_exif(_path_to_image: PathBuf) -> HashMap<String, String> {
    let mut map_data = HashMap::new();

    if let Ok(file) = std::fs::File::open(_path_to_image.as_path()) {
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();

        if let Ok(exif) = exifreader.read_from_container(&mut bufreader) {
            for f in exif.fields() {
                let mut value = f.display_value().to_string();
                if value.starts_with("\"") && value.ends_with("\"") {
                    value = value.strip_prefix("\"").unwrap().strip_suffix("\"").unwrap().to_string();
                }
                map_data.insert(f.tag.to_string(), value);
            }
        }
    }
    return map_data;
}

fn get_file_list(_path: PathBuf) ->  Vec<PathBuf>{
    let mut file_list = vec![];

    for file in fs::read_dir(_path).unwrap() {
        file_list.push(
            file.unwrap().path()
        );
    }

    return file_list;
}