extern crate iron;
extern crate handlebars_iron as hbs;
extern crate serde_json;

#[macro_use]
extern crate maplit;

mod routes;
mod controllers;

use iron::prelude::*;

fn main() {
    let mut chain = Chain::new(routes::all());
    chain.link_after(routes::templates());

    Iron::new(chain).http("localhost:3000").unwrap();
    println!("Running your server <3");
}
