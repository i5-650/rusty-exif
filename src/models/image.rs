pub use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[derive(Serialize, Deserialize)]
pub struct Image {
	pub name: String,
	pub exifs: HashMap<String, BTreeMap<String, String>>,
}
