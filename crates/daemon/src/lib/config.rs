use anyhow::Context;
use log::{debug, warn};
use serde::{
	Deserialize,
	Serialize,
};
use std::{collections::HashMap, default::Default, fs::File, path::{Path, PathBuf}};
use serde_yaml;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
	#[serde(default = "Config::default_feed_path")]
	/// Directory to store all feed related files in
	pub feed_path: String,
	#[serde(default = "Config::latest_version")]
	/// Config version
	pub version:    usize,
	#[serde(default = "Config::default_logpath")]
	/// Directory to place log files
	pub log_path:   String,
	#[serde(flatten)]
	/// serde unkown fields
	pub extra:      HashMap<String, serde_yaml::Value>,
}

impl Default for Config {
	fn default() -> Self {
		return Config {
			feed_path: Self::default_feed_path(),
			version:    Self::latest_version(),
			log_path:   Self::default_logpath(),
			extra:      HashMap::new(),
		};
	}
}

impl Config {
	pub fn default_feed_path() -> String {
		return "./feed".to_string();
	}

	pub fn latest_version() -> usize {
		return 0;
	}

	pub fn default_logpath() -> String {
		return "./log".to_string();
	}

	pub fn from_cli_matches(cli_matches: &clap::ArgMatches) -> anyhow::Result<Self> {
		let config_path = PathBuf::from(&cli_matches.value_of("config").unwrap());

		let config = Self::read_or_create_config(&config_path)?;

		if !config.extra.is_empty() {
			for key in config.extra.keys() {
				warn!("Unkown key in config: \"{}\"", &key);
			}
		}

		return Ok(config);
	}

	fn read_or_create_config(config_path: &PathBuf) -> anyhow::Result<Self> {
		debug!("Config Path is {:?}", &config_path);

		let config: Self;
		if Path::new(&config_path).exists() {
			debug!("Config File Exists");
			let config_raw = File::open(&config_path).context("Opening config path for reading Failed")?;
			config = serde_yaml::from_reader(&config_raw).context("Couldnt read config")?;
		} else {
			debug!("Config File does not exist");
			config = Config::default();
			let write = File::create(&config_path).context("Opening config path for writing Failed")?;
			serde_yaml::to_writer(&write, &config).context("Writing default config failed")?; 
		};

		debug!("Config File's Content: {:#?}", &config);

		return Ok(config);
	}
}
