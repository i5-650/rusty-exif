pub use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Image { pub name: String, pub exifs: HashMap<String, String> }

#[derive(Serialize, Deserialize)]
pub struct Data(pub Vec<Image>);