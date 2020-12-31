pub mod config;
pub mod error;
pub mod routes;

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let v = 4;
		assert_eq!(v, 4);
	}
}
