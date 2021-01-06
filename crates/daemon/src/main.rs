mod lib;
use std::net::SocketAddrV4;

use anyhow::Context;
use clap::load_yaml;
use clap::App;
use flexi_logger::Logger;
use lib::*;
use log::{
	debug,
	info,
};
use tokio::time;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	Logger::with_env_or_str("debug")
		.start()
		.context("Logger failed to start")?; // TODO: change this to "error" when ready
	let yml = load_yaml!("../cli.yml");
	let cli_matches = App::from_yaml(yml).get_matches();
	let config = config::Config::from_cli_matches(&cli_matches).context("Creating config from cli/env failed")?;

	tokio::task::spawn(async move {
		debug!("inside sleep-interval task");
		loop {
			time::sleep(time::Duration::from_secs(60)).await;
			debug!("sleep-interval");
		}
	});

	warp::serve(routes::get_routes(&config))
		.run(SocketAddrV4::new(config.ip, config.port))
		.await;

	info!("daemon end");
	return Ok(());
}
