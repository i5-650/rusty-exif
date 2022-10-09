extern crate exif;

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::fs::File;

const ERROR_MESSAGE: &str = "Missing parameter(s)\n\nUsage:\nrusty-exif inputdir/ output-file.json";

fn main() {
    let args_folder = std::env::args().nth(1).expect(ERROR_MESSAGE);
    let output_filename = std::env::args().nth(2).expect(ERROR_MESSAGE);
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
    string_data += "{\n\t\"datas\": {\n\t\t\"images\": [\n";
    let files_list = get_file_list(_path);

    for i in 0..files_list.len() {
        string_data += "\t\t\t{\n";
        string_data += format!("\t\t\t\t\"image_name\": \"{}\",\n", files_list[i].as_path().display()).as_str();

        if let Ok(file) = std::fs::File::open(files_list[i].as_path()) {
            let mut bufreader = std::io::BufReader::new(&file);
            let exifreader = exif::Reader::new();

            if let Ok(exif) = exifreader.read_from_container(&mut bufreader) {
                for f in exif.fields() {
                    let key = f.tag;
                    let value = String::from(f.display_value().to_string());
                    string_data += format!("\t\t\t\t\"{}\": {},\n", key, add_quote_if_needed(value)).as_str();
                }
            }
            string_data.remove(string_data.len()-2);
            string_data += "\t\t\t}";
        }
        if i != files_list.len() - 1 {
            string_data += ",\n";
        }
        
    }
    string_data += "\n\t\t]\n\t}\n}";
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

fn add_quote_if_needed(value: String) -> String {
    if !value.starts_with("\"") {
        return String::from("\"") + &value + "\"";
    }
    
    return value;
}