use rocket_contrib::json::Json;
use crate::DbConn;
use super::user_model::{NewUser, User};
use super::user_repo as repo;
use diesel::QueryResult;

#[get("/")]
pub fn get_users(conn: DbConn) -> QueryResult<Json<Vec<User>>> {
    repo::list(&conn).map(|users| Json(users))
}

#[get("/<id>")]
pub fn get_user(id: i32, conn: DbConn) -> QueryResult<Json<User>> {
    repo::get(id, &conn).map(|user| Json(user))
}

#[get("/name/<name>")]
pub fn get_user_by_name(name: String, conn: DbConn) -> QueryResult<Json<User>> {
    repo::get_by_name(name, &conn).map(|user| Json(user))
}

#[post("/", format="json", data="<user>")]
pub fn create_user(user: Json<NewUser>, conn: DbConn) -> QueryResult<Json<usize>> {
    let address = user.address.as_ref().unwrap();
    println!("request to create user: {} placed at {}", user.name, address);
    repo::create(user.into_inner(), &conn).map(|count| Json(count))
}

#[put("/", format="json", data="<user>")]
pub fn update_user(user: Json<User>, conn: DbConn) -> QueryResult<Json<usize>> {
    repo::update(user.into_inner(), &conn).map(|count| Json(count))
}

#[delete("/<id>")]
pub fn delete_user(id: i32, conn: DbConn) -> QueryResult<Json<usize>> {
    repo::delete(id, &conn).map(|count| Json(count))
}