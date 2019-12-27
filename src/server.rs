use crate::config;
use crate::database::DbConn;
use crate::graphql::{Mutations, Query, Schema};
use crate::handler::auth::*;
use crate::handler::graphql::*;
use crate::handler::ping::*;

use rocket::config::{Config, Environment, Value};
use rocket::fairing::AdHoc;
use rocket::http::hyper::header;
use std::collections::HashMap;

pub struct Server {
    config: Config,
}

impl Server {
    pub fn new() -> Self {
        let mut database_config = HashMap::new();
        let mut databases = HashMap::new();
        database_config.insert("url", Value::from(config::database_url()));
        databases.insert("postgres_db", Value::from(database_config));

        let config = Config::build(Environment::Staging)
            .port(config::port().parse().unwrap())
            .extra("databases", databases)
            .finalize()
            .unwrap();

        Server { config }
    }

    pub fn init(self) -> rocket::Rocket {
        rocket::custom(self.config)
            .attach(DbConn::fairing())
            .attach(AdHoc::on_response("Enable CORS", |req, res| {
                res.set_header(header::AccessControlAllowOrigin::Any);
            }))
            .manage(Schema::new(Query, Mutations))
            .mount("/ping", routes![ping])
            .mount("/auth", routes![register, login])
            .mount("/graphql", rocket::routes![post_graphql_handler, graphiql])
    }
}
