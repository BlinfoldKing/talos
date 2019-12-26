use crate::database::DbConn;
use juniper::Context;

impl Context for DbConn {}

pub struct Query;
pub struct Mutations;

// list of resolvers
pub mod post_resolver;
