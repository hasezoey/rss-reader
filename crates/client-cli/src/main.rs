use clap::load_yaml;
use clap::App;
use flexi_logger::Logger;
use log::debug;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	Logger::with_env_or_str("debug").start()?; // TODO: change this to "error" when ready
	let yml = load_yaml!("../cli.yml");
	let _cli_matches = App::from_yaml(yml).get_matches();

	debug!("Hello from Client (Debug)");
	println!("Hello from Client");
	
	return Ok(());
}
