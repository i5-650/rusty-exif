use std::path::PathBuf;
use std::{fs, collections::HashMap};

mod data_structures;

pub fn map_exif(_path_to_image: PathBuf) -> HashMap<String, String> {
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


pub fn json_string_from_dir(_path: PathBuf) -> data_structures::Data {
    let files_list = get_file_list(_path);
    let mut data_out = data_structures::Data { images: vec![] };

    for pathbuf_file in files_list {
        data_out.images.push(data_structures::Image {
            image_name: pathbuf_file.as_path().display().to_string(), 
            exif_fields: map_exif(pathbuf_file)});
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
