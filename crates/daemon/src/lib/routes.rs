use warp::{
	filters::BoxedFilter,
	Filter,
	Reply,
};

pub fn get_routes(_config: &crate::config::Config) -> BoxedFilter<(impl Reply,)> {
	let root_path = warp::path::end().map(|| "Hello Root");
	let test1 = warp::path::end().map(|| "Hello Test1");
	let test2 = warp::path("nested").and(warp::path::end()).map(|| "Hello test2");

	let testy = warp::path("test").and(test1.or(test2));
	return warp::get().and(root_path.or(testy)).with(warp::log("*")).boxed();
}
