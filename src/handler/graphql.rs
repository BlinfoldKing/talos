use rocket::{response::content, State};

use crate::database::DbConn;
use crate::graphql::{Mutations, Query};
use juniper::RootNode;

pub type Schema = RootNode<'static, Query, Mutations>;

pub struct Context {
    db: DbConn,
    schema: Schema,
}

#[rocket::post("/graphql", data = "<request>")]
pub fn post_graphql_handler(
    context: DbConn,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[rocket::get("/graphql?<request>")]
pub fn get_graphql_handler(
    context: DbConn,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[rocket::get("/")]
pub fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}
