extern crate exif;

use std::{fs, collections::HashMap};
use std::io::Write;
use std::path::PathBuf;
use std::fs::File;
use serde::{Serialize, Deserialize};

const ERROR_MESSAGE: &str = "Missing parameter(s)\n\nUsage:\nrusty-exif inputdir/ output-file.json";

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
    let args_folder = std::env::args().nth(1).expect(ERROR_MESSAGE);
    let output_filename = std::env::args().nth(2).expect(ERROR_MESSAGE);
    let predic = File::create(output_filename);

    if predic.is_ok() {
        let mut output_file = predic.unwrap();
        let data_to_jsonify = json_string_from_dir(PathBuf::from(args_folder));
        let output = serde_json::to_string_pretty(&data_to_jsonify);
        if output_file.write(output.unwrap().as_bytes()).is_ok() {
            println!("All good");
        }
    }
    
}

fn json_string_from_dir(_path: PathBuf) -> Data {
    let files_list = get_file_list(_path);
    let mut data_out = Data { images: Vec::new() };

    for pathbuf_file in files_list {
        if let Ok(file) = std::fs::File::open(pathbuf_file.as_path()) {
            let mut bufreader = std::io::BufReader::new(&file);
            let exifreader = exif::Reader::new();

            if let Ok(exif) = exifreader.read_from_container(&mut bufreader) {
                let mut fields_map :HashMap<String, String> = HashMap::new();
                for f in exif.fields() {
                    fields_map.insert(f.tag.to_string(), f.display_value().to_string());
                }
                data_out.images.push(Image {image_name: pathbuf_file.as_path().display().to_string(), exif_fields: fields_map})
            }
        }
    }
    return data_out;
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