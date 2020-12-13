use log::debug;
use rss_reader_daemon_core::config::Config;
use serde::{
	Deserialize,
	Serialize,
};
use std::default::Default;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct BinConfig {
    /// Config to be passed to the lib
    lib_config: Config,
    /// Directory to place log files
    log_path: String,
}

impl Default for BinConfig {
	fn default() -> Self {
		return BinConfig {
			log_path: "./log".to_string(),
			lib_config: Config::default(),
		}
	}
}

impl BinConfig {
	pub fn from_cli_matches(cli_matches: &clap::ArgMatches) -> Result<BinConfig, Box<dyn Error>> {
		let config_path = PathBuf::from(&cli_matches.value_of("config").unwrap());
		debug!("Config Path is {:?}", &config_path);

		let config: BinConfig = match Path::new(&config_path).exists() {
			// is there an better way to do this?
			true => {
				debug!("Config File Exists");
				let config_raw = File::open(&config_path).expect("Opening config path for reading Failed");
				serde_yaml::from_reader(&config_raw).expect("Couldnt read config")
			},
			false => {
				debug!("Config File does not exist");
				let config = BinConfig::default();
				let write = File::create(&config_path).expect("Opening config path for writing Failed");
				serde_yaml::to_writer(&write, &config).expect("Writing default config failed");
				config
			},
		};

		debug!("Config File's Content: {:#?}", &config);
		return Ok(config);
	}
}
