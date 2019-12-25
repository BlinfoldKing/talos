extern crate diesel;
extern crate juniper;

use crate::database::schema::posts;
use crate::database::DbConn;
use chrono::prelude::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error as DBError;
use diesel::RunQueryDsl;
use uuid::Uuid;

#[derive(Queryable, juniper::GraphQLObject)]
pub struct Post {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub banner: String,
    pub is_draft: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub id: Uuid,
    pub slug: &'a str,
    pub title: &'a str,
    pub content: &'a str,
    pub banner: &'a str,
    pub is_draft: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Post {
    pub fn new<'a>(db: &DbConn, title: &'a str) -> Result<Post, DBError> {
        let word_list: Vec<&str> = title.split(' ').collect();
        let slug: &str = &word_list.join("-");
        let now = chrono::Local::now().naive_local();
        diesel::insert_into(posts::table)
            .values(&NewPost {
                id: uuid::Uuid::new_v4(),
                title,
                slug,
                content: "Another New Story",
                banner: "",
                is_draft: true,
                created_at: now,
                updated_at: now,
            })
            .get_result::<Post>(&**db)
    }

    pub fn find_by_id(db: &DbConn, _id: Uuid) -> Result<Option<Post>, DBError> {
        use crate::database::schema::posts::dsl::*;
        posts.find(_id).first::<Post>(&**db).optional()
    }
}
