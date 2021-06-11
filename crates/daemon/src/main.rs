mod lib;
use std::net::SocketAddrV4;

use anyhow::Context;
use clap::load_yaml;
use clap::App;
use flexi_logger::{
	writers::FileLogWriter,
	Age,
	Cleanup,
	Criterion,
	FileSpec,
	Logger,
	Naming,
};
use lib::*;
use log::{
	debug,
	info,
};
use tokio::time;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	// TODO: change this "log_to_file" somehow to not actually write until reconfigured
	let logger_handle = Logger::try_with_env_or_str("debug")?
		.log_to_file(FileSpec::default().directory(std::env::temp_dir()))
		.duplicate_to_stderr(flexi_logger::Duplicate::All)
		.start()
		.context("Logger failed to start")?; // TODO: change this to "error" when ready
	let yml = load_yaml!("../cli.yml");
	let cli_matches = App::from_yaml(yml).get_matches();
	let config = config::Config::from_cli_matches(&cli_matches).context("Creating config from cli/env failed")?;

	let flwb = {
		let filespec = FileSpec::default().directory(&config.log_path).suffix("log");
		FileLogWriter::builder(filespec)
			.rotate(
				Criterion::AgeOrSize(Age::Day, 1024 * 1024 * 10), // 10mb
				Naming::Timestamps,
				Cleanup::KeepLogAndCompressedFiles(3, 10),
			)
			.print_message()
			.cleanup_in_background_thread(true)
			.append()
	};
	logger_handle
		.reset_flw(&flwb)
		.context("Logger failed to reconfigure to Config")?;

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
	logger_handle.shutdown();

	return Ok(());
}
