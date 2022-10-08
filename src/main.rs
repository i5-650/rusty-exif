extern crate exif;

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::fs::File;

fn main() {
    let args_folder = std::env::args().nth(1).expect("Missing parameter(s)\n\nUsage:\nrusty-exif inputdir/ output-file.json");
    let output_filename = std::env::args().nth(2).expect("Missing parameter(s)\n\nUsage:\nrusty-exif inputdir/ output-file.json");
    let predic = File::create(output_filename);

    if predic.is_ok() {
        let mut output_file = predic.unwrap();
        let output_string = json_string_from_dir(PathBuf::from(args_folder));
        if output_file.write(output_string.as_bytes()).is_ok() {
            println!("All good");
        }
    }
    
}

fn json_string_from_dir(_path: PathBuf) -> String {
    let mut string_data = String::new();
    string_data += "{ \"datas\": { \"images\": [ ";
    let files_list = get_file_list(_path);

    for i in 0..files_list.len() {
        string_data += "{";
        string_data += format!("\"image_name\": \"{}\",", files_list[i].as_path().display()).as_str();

        if let Ok(file) = std::fs::File::open(files_list[i].as_path()) {
            let mut bufreader = std::io::BufReader::new(&file);
            let exifreader = exif::Reader::new();

            if let Ok(exif) = exifreader.read_from_container(&mut bufreader) {
                for f in exif.fields() {
                    let key = f.tag;
                    let value = String::from(f.display_value().to_string());
                    string_data += format!("\"{}\": {}, ", key, value).as_str();
                }
            }
            string_data += "}";
        }
        if i != files_list.len() - 1 {
            string_data += ",";
        }
        
    }
    string_data += "]}";
    return string_data;
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