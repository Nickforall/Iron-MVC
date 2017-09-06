extern crate router;

use self::router::Router;
use super::controllers::StaticController;

use hbs::{HandlebarsEngine, DirectorySource, MemorySource};

pub fn all() -> Router {
	let mut router = Router::new();
    router.get("/", StaticController::index, "index");
    router
}

pub fn templates() -> HandlebarsEngine {
	// create handlebars engine
	let mut hbse = HandlebarsEngine::new();
	// add a directory source, all files with .hbs suffix will be loaded as template
	hbse.add(Box::new(DirectorySource::new("./src/views", ".hbs")));

	let mem_templates = btreemap! {
		"index".to_owned() => include_str!("../views/index.hbs").to_owned()
	};

	// add a memory based source
	hbse.add(Box::new(MemorySource(mem_templates)));

	// load templates from all registered sources
	if let Err(r) = hbse.reload() {
		panic!("{}", r);
	}

	return hbse;
}
