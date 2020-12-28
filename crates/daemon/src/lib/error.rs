use thiserror::Error;

#[derive(Debug, Error)]
pub enum Errors {
	#[error("hello ioerror")]
	Io(#[from] std::io::Error),

	#[error("hello parseerror")]
	Parse(#[from] std::string::ParseError),
}
