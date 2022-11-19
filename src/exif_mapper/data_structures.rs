use serde::{Serialize, Deserialize};
use std::{collections::HashMap};

#[derive(Serialize, Deserialize)]
pub struct Image {
    pub image_name: String,
    #[serde(flatten)]
    pub exif_fields: HashMap<String, String>
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub images: Vec<Image>
}