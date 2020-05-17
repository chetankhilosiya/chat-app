use rocket_contrib::json::Json;
use crate::{DbConn, user::user_controller};

#[post("/user/<user_name>")]
pub fn auth_user(user_name: String, conn: DbConn) -> Json<bool> {
    match user_controller::get_user_by_name(user_name, conn) {
        Ok(_user) => Json(true),
        Err(_) => Json(false)
    }
}
