use super::{Mutations, Query};
use crate::database::DbConn;
use crate::domain::post::{CreatePostForm, Post, UpdatePostForm};
use juniper::FieldResult;

#[derive(juniper::GraphQLInputObject)]
struct CreatePostInput {
    pub slug: Option<String>,
    pub title: String,
    pub content: Option<String>,
    pub banner: Option<String>,
    pub is_draft: bool,
}

#[derive(juniper::GraphQLInputObject)]
struct UpdatePostInput {
    pub slug: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub banner: Option<String>,
    pub is_draft: Option<bool>,
}

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
            Err(_) => None,
        }
    }

    fn GetAllPost(database: &DbConn) -> Option<Vec<Post>> {
        let result = Post::get_all(database).unwrap();
        result
    }
}

#[juniper::object(
    Context = DbConn,
    Scalar = juniper::DefaultScalarValue,
)]
impl Mutations {
    fn CreatePost(database: &DbConn, create_post_input: CreatePostInput) -> FieldResult<Post> {
        let title = &create_post_input.title;
        let slug = match create_post_input.slug {
            Some(s) => s,
            None => {
                let word_list: Vec<&str> = title.split(' ').collect();
                word_list.join("-")
            }
        };
        let content = create_post_input.content.as_deref();
        let banner = create_post_input.banner.as_deref();
        let is_draft = create_post_input.is_draft;
        Ok(Post::new(
            database,
            CreatePostForm {
                title,
                is_draft,
                slug: &slug,
                banner: match banner {
                    Some(b) => b,
                    None => "",
                },
                content: match content {
                    Some(c) => c,
                    None => "",
                },
            },
        )
        .unwrap())
    }

    fn UpdatePostById(
        database: &DbConn,
        id: uuid::Uuid,
        update_post_input: UpdatePostInput,
    ) -> FieldResult<Post> {
        let slug = update_post_input.slug.as_deref();
        let title = update_post_input.title.as_deref();
        let content = update_post_input.content.as_deref();
        let banner = update_post_input.banner.as_deref();
        let is_draft = update_post_input.is_draft;
        Ok(Post::update_by_id(
            database,
            id,
            UpdatePostForm {
                slug,
                title,
                content,
                banner,
                is_draft,
            },
        )
        .unwrap())
    }
}
