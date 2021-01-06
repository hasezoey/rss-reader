use warp::{
	filters::BoxedFilter,
	Filter,
	Reply,
};

pub fn get_routes(_config: &crate::config::Config) -> BoxedFilter<(impl Reply,)> {
	return warp::any().map(|| return "Hello there").boxed();
}
