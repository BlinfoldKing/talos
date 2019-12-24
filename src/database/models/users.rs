#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub username: String,
    pub password_hash: bool,
}
