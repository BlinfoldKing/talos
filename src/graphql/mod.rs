use crate::database::DbConn;
use crate::domain::{post::Post};
use juniper::{Context, FieldResult};

impl Context for DbConn{}

pub struct Query;

#[juniper::object(
    Context = DbConn,
    Scalar = juniper::DefaultScalarValue,
)]
impl Query {
    #[graphql(arguments(id(description = "id of the post")))]
    fn post(database: &DbConn, id: uuid::Uuid) -> Option<Post> {
        let result = Post::find_by_id(database, id);
        match result {
            Ok(opt) => opt,
            Err(_) => None
        }
    }
}

pub struct Mutations;
#[juniper::object(
    Context = DbConn, 
    Scalar = juniper::DefaultScalarValue,
)]
impl Mutations {
    fn newPost(database: &DbConn, title: String) -> FieldResult<Post> {
       Ok(Post::new(database, &title).unwrap())
    }
}
