extern crate bcrypt;

use crate::database::DbConn;
use crate::domain::user::*;
use rocket::response::content;
use rocket_contrib::json::Json;

#[derive(Deserialize)]
pub struct RegisterRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[post("/register", data = "<form>")]
pub fn register(db: DbConn, form: Json<RegisterRequest>) -> content::Json<String> {
    let new_user = User::new(db, &form.username, &form.password);
    let response = format!(
        "{{ \"ok\": \"true\", \"token\": \"{}\" }}",
        new_user.generate_token()
    );
    content::Json(response)
}

#[post("/login", data = "<form>")]
pub fn login(db: DbConn, form: Json<LoginRequest>) -> content::Json<String> {
    let user = User::get(db, &form.username);
    if let Some(u) = user {
        if u.verify_password(&form.password) {
            let response = format!(
                "{{ \"ok\": \"true\", \"token\": \"{}\" }}",
                u.generate_token()
            );
            content::Json(response)
        } else {
            content::Json("\"error\": \"some error\"".to_owned())
        }
    } else {
        content::Json("\"error\": \"some error\"".to_owned())
    }
}
