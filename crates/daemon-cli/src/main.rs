use clap::load_yaml;
use clap::App;
use std::error::Error;
use flexi_logger::Logger;

mod config;

fn main() -> Result<(), Box<dyn Error>> {
	Logger::with_env_or_str("debug").start()?; // TODO: change this to "error" when ready
	let yml = load_yaml!("../cli.yml");
	let cli_matches = App::from_yaml(yml).get_matches();
	let _config = config::BinConfig::from_cli_matches(&cli_matches)?;

	println!("Hello from Server");
	return Ok(());
}
