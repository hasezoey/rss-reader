use serde::{Serialize, Deserialize};
use std::default::Default;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
	#[serde(default = "Config::default_feed_path")]
    /// Directory to store all feed related files in
    pub feed_path: String,
}

impl Default for Config {
	fn default() -> Self {
		Config {
			feed_path: Self::default_feed_path(),
		}
	}
}

impl Config {
	pub fn default_feed_path() -> String {
		return "./feed".to_string();
	}
}
