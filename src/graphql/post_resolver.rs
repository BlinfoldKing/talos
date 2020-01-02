use super::{GQLContext, Mutations, Query};
use crate::domain::post::{CreatePostForm, Post, UpdatePostForm};
use juniper::{graphql_value, FieldError, FieldResult};

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
    Context = GQLContext,
    Scalar = juniper::DefaultScalarValue,
)]
impl Post {
    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    pub fn slug(&self) -> String {
        self.slug.clone()
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn content(&self) -> String {
        self.content.clone()
    }

    pub fn banner(&self) -> String {
        self.banner.clone()
    }

    pub fn is_draft(&self) -> bool {
        self.is_draft
    }

    pub fn created_at(&self) -> chrono::NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> chrono::NaiveDateTime {
        self.updated_at
    }

    pub fn deleted_at(&self) -> Option<chrono::NaiveDateTime> {
        self.deleted_at
    }

    pub fn next(&self, ctx: &GQLContext) -> Option<Post> {
        match self.next_id {
            Some(id) => Post::find_by_id(&ctx.database, id).unwrap_or(None),
            None => None,
        }
    }

    pub fn prev(&self, ctx: &GQLContext) -> Option<Post> {
        match self.prev_id {
            Some(id) => Post::find_by_id(&ctx.database, id).unwrap(),
            None => None,
        }
    }

    pub fn preview(&self) -> String {
        let res: Vec<&str> = self.content.split(' ').collect();
        let limit = if res.len() < 100 { res.len() } else { 100 };
        format!("{}...", res[..limit - 1].join(" ").to_owned())
    }
}

#[juniper::object(
    Context = GQLContext,
    Scalar = juniper::DefaultScalarValue,
)]
impl Query {
    #[graphql(arguments(id(description = "id of the post")))]
    fn post(ctx: &GQLContext, id: uuid::Uuid) -> Option<Post> {
        let result = Post::find_by_id(&ctx.database, id);
        match result {
            Ok(opt) => opt,
            Err(_) => None,
        }
    }

    fn FindPostBySlug(ctx: &GQLContext, slug: String) -> Option<Post> {
        let result = Post::find_by_slug(&ctx.database, slug);
        match result {
            Ok(opt) => opt,
            Err(_) => None,
        }
    }

    fn SearchPostByTitle(
        ctx: &GQLContext,
        query: String,
        limit: i32,
        offset: i32,
    ) -> Option<Vec<Post>> {
        let result = Post::search_by_title(&ctx.database, query, limit as i64, offset as i64);
        match result {
            Ok(opt) => opt,
            Err(_) => None,
        }
    }

    fn GetAllPosts(ctx: &GQLContext, limit: i32, offset: i32) -> Option<Vec<Post>> {
        let result = Post::get_all(&ctx.database, limit as i64, offset as i64).unwrap();
        result
    }
}

#[juniper::object(
    Context = GQLContext,
    Scalar = juniper::DefaultScalarValue,
)]
impl Mutations {
    fn CreatePost(ctx: &GQLContext, create_post_input: CreatePostInput) -> FieldResult<Post> {
        if ctx.user.is_none() {
            return Err(FieldError::new(
                "Token Needed",
                graphql_value!({"unauthorised": "no token existed"}),
            ));
        };

        let title = &create_post_input.title;
        let mut slug = match create_post_input.slug {
            Some(s) => s,
            None => {
                let word_list: Vec<&str> = title.split(' ').collect();
                format!("{}_{}", nanoid::generate(21), word_list.join("-"))
            }
        };

        if let Ok(_) = Post::find_by_slug(&ctx.database, slug.clone()) {
            let word_list: Vec<&str> = title.split(' ').collect();
            slug = word_list.join("-");
            slug = format!("{}_{}", nanoid::generate(21), slug);
        };

        let content = create_post_input.content.as_deref();
        let banner = create_post_input.banner.as_deref();
        let is_draft = create_post_input.is_draft;
        Ok(Post::new(
            &ctx.database,
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
        ctx: &GQLContext,
        id: uuid::Uuid,
        update_post_input: UpdatePostInput,
    ) -> FieldResult<Post> {
        if ctx.user.is_none() {
            return Err(FieldError::new(
                "Token Needed",
                graphql_value!({"unauthorised": "no token existed"}),
            ));
        };

        let slug = update_post_input.slug.as_deref();
        let title = update_post_input.title.as_deref();
        let content = update_post_input.content.as_deref();
        let banner = update_post_input.banner.as_deref();
        let is_draft = update_post_input.is_draft;
        Ok(Post::update_by_id(
            &ctx.database,
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
