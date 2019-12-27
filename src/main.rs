#![feature(proc_macro_hygiene, decl_macro)]
use rocket::config::{Config, Environment, Value};
use std::collections::HashMap;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_json;

mod config;
mod database;
mod domain;
mod graphql;
mod handler;

use crate::graphql::{Mutations, Query};
use handler::graphql::*;

fn main() {
    config::init();
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    database_config.insert("url", Value::from(config::database_url()));
    databases.insert("postgres_db", Value::from(database_config));

    let config = Config::build(Environment::Staging)
        .port(config::port().parse().unwrap())
        .extra("databases", databases)
        .finalize()
        .unwrap();

    rocket::custom(config)
        .attach(database::DbConn::fairing())
        .manage(handler::graphql::Schema::new(Query, Mutations))
        .mount("/ping", routes![handler::ping::ping])
        .mount(
            "/auth",
            routes![handler::auth::register, handler::auth::login],
        )
        .mount("/graphql", rocket::routes![post_graphql_handler, graphiql])
        .launch();
}
