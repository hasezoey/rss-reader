use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Errors {
	#[error("hello ioerror")]
	Io(#[from] std::io::Error),

	#[error("hello parseerror")]
	Parse(#[from] std::string::ParseError),

	#[error("LockFile already exists at \"{0}\", by process \"{1}\"")]
	LockFailed(PathBuf, usize),
}
