extern crate diesel;
extern crate juniper;

use crate::database::schema::posts;
use crate::database::DbConn;
use chrono::prelude::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error as DBError;
use diesel::RunQueryDsl;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Post {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub banner: String,
    pub is_draft: bool,
    pub prev_id: Option<Uuid>,
    pub next_id: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct CreatePostForm<'a> {
    pub slug: &'a str,
    pub title: &'a str,
    pub content: &'a str,
    pub banner: &'a str,
    pub is_draft: bool,
}

#[derive(AsChangeset)]
#[table_name = "posts"]
pub struct UpdatePostForm<'a> {
    pub slug: Option<&'a str>,
    pub title: Option<&'a str>,
    pub content: Option<&'a str>,
    pub banner: Option<&'a str>,
    pub is_draft: Option<bool>,
}

impl Post {
    pub fn new<'a>(db: &DbConn, new_post: CreatePostForm) -> Result<Post, DBError> {
        use crate::database::schema::posts::dsl::*;
        let now = chrono::Local::now().naive_local();

        let prev = posts
            .order_by(created_at.desc())
            .first::<Post>(&**db)
            .optional()
            .unwrap_or(None);

        let _prev_id = match prev {
            Some(post) => Some(post.id),
            None => None,
        };

        let res = diesel::insert_into(posts)
            .values((
                id.eq(uuid::Uuid::new_v4()),
                &new_post,
                created_at.eq(now),
                updated_at.eq(now),
                prev_id.eq(_prev_id),
            ))
            .get_result::<Post>(&**db);

        if let Ok(post) = res {
            match _prev_id {
                Some(_id) => {
                    let _ = diesel::update(posts.find(_id))
                        .set((next_id.eq(post.id), updated_at.eq(now)))
                        .get_result::<Post>(&**db);
                    let mut result_post = post;
                    result_post.prev_id = Some(_id);
                    Ok(result_post)
                }
                None => Ok(post),
            }
        } else {
            res
        }

        // res
    }

    pub fn find_by_id(db: &DbConn, _id: Uuid) -> Result<Option<Post>, DBError> {
        use crate::database::schema::posts::dsl::*;
        posts.find(_id).first::<Post>(&**db).optional()
    }

    pub fn find_by_slug(db: &DbConn, _slug: String) -> Result<Option<Post>, DBError> {
        use crate::database::schema::posts::dsl::*;
        posts.filter(slug.eq(_slug)).first::<Post>(&**db).optional()
    }

    pub fn get_all(db: &DbConn, _limit: i64, _offset: i64) -> Result<Option<Vec<Post>>, DBError> {
        use crate::database::schema::posts::dsl::*;
        posts
            .order_by(created_at.desc())
            .limit(_limit)
            .offset(_offset)
            .load::<Post>(&**db)
            .optional()
    }

    pub fn update_by_id(db: &DbConn, _id: Uuid, update: UpdatePostForm) -> Result<Post, DBError> {
        use crate::database::schema::posts::dsl::*;
        let now = chrono::Local::now().naive_local();
        diesel::update(posts.find(_id))
            .set((&update, updated_at.eq(now)))
            .get_result::<Post>(&**db)
    }
}
