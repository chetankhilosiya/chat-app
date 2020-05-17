use rocket::*;
use super::message_model::{NewTextMessage, TextMessage};
use super::message_repo as repo;
use rocket_contrib::json::Json;
use crate::DbConn;
use diesel::QueryResult;

#[get("/<user_id>")]
pub fn get_messages(user_id: i32, conn: DbConn) -> QueryResult<Json<Vec<TextMessage>>> {
    repo::list_to_user(user_id, &conn)
        .map(|messages| Json(messages))
}

#[post("/", format="json", data="<message>")]
pub fn create_message(message: Json<NewTextMessage>, conn: DbConn) -> QueryResult<Json<usize>> {
    repo::create(message.into_inner(), &conn)
        .map(|count| Json(count))
}

#[put("/", format="json", data="<message>")]
pub fn update_message(message: Json<TextMessage>, conn: DbConn) -> QueryResult<Json<usize>> {
    repo::update(message.into_inner(), &conn)
        .map(|count| Json(count))
}

#[delete("/<message_id>")]
pub fn delete_message(message_id: i32, conn: DbConn) -> QueryResult<Json<usize>> {
    repo::delete(message_id, &conn).map(|count| Json(count))
}