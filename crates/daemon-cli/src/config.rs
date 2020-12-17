use anyhow::Context;
use log::{
	debug,
	warn,
};
use rss_reader_daemon_core::config::Config;
use serde::{
	Deserialize,
	Serialize,
};
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::{
	collections::HashMap,
	default::Default,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BinConfig {
	#[serde(default = "BinConfig::latest_version")]
	/// Config version
	pub version:    usize,
	#[serde(flatten)]
	/// Config to be passed to the lib
	pub lib_config: Config,
	#[serde(default = "BinConfig::default_logpath")]
	/// Directory to place log files
	pub log_path:   String,
	#[serde(flatten)]
	/// serde unkown fields
	pub extra:      HashMap<String, serde_yaml::Value>,
}

impl Default for BinConfig {
	fn default() -> Self {
		return Self {
			version:    Self::latest_version(),
			log_path:   Self::default_logpath(),
			lib_config: Config::default(),
			extra:      HashMap::new(),
		};
	}
}

impl BinConfig {
	pub fn latest_version() -> usize {
		return 0;
	}

	pub fn default_logpath() -> String {
		return "./log".to_string();
	}

	pub fn from_cli_matches(cli_matches: &clap::ArgMatches) -> anyhow::Result<Self> {
		let config_path = PathBuf::from(&cli_matches.value_of("config").unwrap());
		debug!("Config Path is {:?}", &config_path);

		let config: BinConfig = if Path::new(&config_path).exists() {
			debug!("Config File Exists");
			let config_raw = File::open(&config_path).context("Opening config path for reading Failed")?;
			serde_yaml::from_reader(&config_raw).context("Couldnt read config")?
		} else {
			debug!("Config File does not exist");
			let config = BinConfig::default();
			let write = File::create(&config_path).context("Opening config path for writing Failed")?;
			serde_yaml::to_writer(&write, &config).context("Writing default config failed")?;
			config
		};

		debug!("Config File's Content: {:#?}", &config);

		if config.extra.is_empty() {
			for key in config.extra.keys() {
				warn!("Unkown key in config: \"{}\"", &key);
			}
		}

		return Ok(config);
	}
}
