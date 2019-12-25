extern crate diesel;

use diesel::pg::PgConnection;

#[database("postgres_db")]
pub struct DbConn(PgConnection);
