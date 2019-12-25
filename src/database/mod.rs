extern crate diesel;
extern crate dotenv;
extern crate rocket_contrib;

use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn establish_connection(db_url: String) -> PgConnection {
    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}

// use rocket_contrib::databases::diesel;

#[database("postgres_db")]
pub struct DbConn(diesel::PgConnection);
