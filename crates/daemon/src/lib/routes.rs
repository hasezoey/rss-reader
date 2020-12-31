use warp::Filter;

pub fn get_routes(&config: crate::config::Config) -> warp::Filter {
	return warp::any().map(|| return "Hello there");
}
