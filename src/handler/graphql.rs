use rocket::{response::content, State};

use juniper::{
    tests::{model::Database, schema::Query},
    EmptyMutation, RootNode,
};

pub type Schema = RootNode<'static, Query, EmptyMutation<Database>>;

#[rocket::post("/graphql", data = "<request>")]
pub fn post_graphql_handler(
    context: State<Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[rocket::get("/graphql?<request>")]
pub fn get_graphql_handler(
    context: State<Database>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[rocket::get("/")]
pub fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}
