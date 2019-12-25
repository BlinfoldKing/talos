extern crate diesel;

use crate::database::DbConn;
use crate::schema::users;
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use diesel::RunQueryDsl;
use frank_jwt::{decode, encode, Algorithm};
use uuid::Uuid;

#[derive(Queryable)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: uuid::Uuid,
    pub username: &'a str,
    pub password_hash: &'a str,
}

impl User {
    pub fn new<'a>(db: DbConn, username: &'a str, password: &'a str) -> User {
        let password_hash = &hash(password, DEFAULT_COST).expect("failed to encrypt password");
        diesel::insert_into(users::table)
            .values(&NewUser {
                id: uuid::Uuid::new_v4(),
                username,
                password_hash,
            })
            .get_result::<User>(&*db)
            .expect("error when saving")
    }

    pub fn get<'a>(db: DbConn, _username: &'a str) -> Option<User> {
        use crate::schema::users::dsl::*;
        let user = users
            .filter(username.eq(_username))
            .first::<User>(&*db)
            .optional();
        if let Ok(Some(u)) = user {
            return Some(u);
        }
        None
    }

    pub fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.password_hash).unwrap()
    }

    pub fn generate_token(&self) -> String {
        let payload = json!({
            "id": format!("{}", self.id),
        });

        let header = json!({});
        let secret = "secret123";
        let jwt = encode(header, &secret, &payload, Algorithm::HS256);
        jwt.expect("token error")
    }
}
