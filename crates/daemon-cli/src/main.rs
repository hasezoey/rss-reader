use clap::load_yaml;
use clap::App;
// use rss_reader_daemon_core::locks::try_lock;
use flexi_logger::Logger;

mod config;

fn main() -> anyhow::Result<()> {
	Logger::with_env_or_str("debug").start()?; // TODO: change this to "error" when ready
	let yml = load_yaml!("../cli.yml");
	let cli_matches = App::from_yaml(yml).get_matches();
	let config = config::BinConfig::from_cli_matches(&cli_matches)?;

	// try_lock(&config.lib_config)?;

	println!("Hello from Server");
	return Ok(());
}
