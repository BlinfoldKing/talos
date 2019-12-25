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
mod handler;
mod schema;

use handler::graphql::*;

use juniper::{
    tests::{model::Database, schema::Query},
    EmptyMutation,
};

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
        .manage(Database::new())
        .manage(handler::graphql::Schema::new(
            Query,
            EmptyMutation::<Database>::new(),
        ))
        .mount("/ping", routes![handler::ping::ping])
        .mount(
            "/auth",
            routes![handler::auth::register, handler::auth::login],
        )
        .mount(
            "/",
            rocket::routes![post_graphql_handler, get_graphql_handler, graphiql],
        )
        .launch();
}
