use serde::{Serialize, Deserialize};
use std::default::Default;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// Directory to store all feed related files in
    feed_path: String,
}

impl Default for Config {
	fn default() -> Self {
		Config {
			feed_path: "./feed".to_string(),
		}
	}
}
