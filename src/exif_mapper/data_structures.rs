pub use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Image {
	pub name: String,
	pub exifs: HashMap<String, String>,
}