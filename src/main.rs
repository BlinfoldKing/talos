#![feature(proc_macro_hygiene, decl_macro)]

use rocket::config::{Config, Environment};
use std::env;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let port = env::var("PORT").unwrap_or("8000".to_owned());
    let config = Config::build(Environment::Staging)
        .port(port.parse().unwrap())
        .finalize()
        .unwrap();

    rocket::custom(config).mount("/", routes![index]).launch();
}
