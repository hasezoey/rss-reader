use clap::load_yaml;
use clap::App;
// use rss_reader_daemon_core::locks::try_lock;
use flexi_logger::Logger;
use anyhow::Context;
use log::info;

mod config;

fn main() -> anyhow::Result<()> {
	Logger::with_env_or_str("debug").start().context("Logger failed to start")?; // TODO: change this to "error" when ready
	let yml = load_yaml!("../cli.yml");
	let cli_matches = App::from_yaml(yml).get_matches();
	let config = config::BinConfig::from_cli_matches(&cli_matches).context("Creating config from cli/env failed")?;

	// try_lock(&config.lib_config)?;

	info!("daemon end");
	return Ok(());
}
