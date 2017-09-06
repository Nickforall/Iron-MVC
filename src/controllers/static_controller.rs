use iron::prelude::*;
use iron::status;
use hbs::handlebars::to_json;
use serde_json::value::{Map};
use hbs::Template;

pub struct StaticController;

impl StaticController {
    pub fn index(_: &mut Request) -> IronResult<Response> {
        let mut data = Map::new();
        data.insert("year".to_string(), to_json(&"2017".to_owned()));
        
        let mut resp = Response::new();
        resp.set_mut(Template::new("index", data)).set_mut(status::Ok);

        Ok(resp)
    }
}
