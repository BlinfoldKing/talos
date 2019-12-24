#![feature(proc_macro_hygiene, decl_macro)]

use rocket::config::{Config, Environment};

#[macro_use]
extern crate rocket;
extern crate diesel;

mod config;
mod database;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    config::init();
    database::establish_connection(config::database_url());
    let config = Config::build(Environment::Staging)
        .port(config::port().parse().unwrap())
        .finalize()
        .unwrap();

    rocket::custom(config).mount("/", routes![index]).launch();
}
