#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

mod schema;
pub mod user;
pub mod message;
mod auth;

use rocket::{Request, Outcome, State};
use rocket::request::{self, FromRequest};
use r2d2;
use r2d2_diesel::ConnectionManager;
use diesel::prelude::*;
use rocket::http::Status;
use dotenv::dotenv;
use std::env;
use std::ops::Deref;
use message::message_controller as message_api;
use user::user_controller as user_api;

type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url());
    Pool::new(manager).expect("db pool")
}

pub fn database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<MysqlConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {

    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, Self::Error> {

        let pool = request.guard::<State<Pool>>()?;

        match pool.get() {

            Ok(conn) => Outcome::Success(DbConn(conn)),

            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),

        }

    }
}

impl Deref for DbConn {

    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target {

        &self.0

    }
}

fn main() {
    dotenv().ok();
    rocket::ignite()
        .manage(init_pool())
        .mount("/users", routes![
            user_api::get_users, user_api::get_user, user_api::get_user_by_name, user_api::create_user,
            user_api::update_user, user_api::delete_user
        ]).mount("/messages", routes![
            message_api::get_messages, message_api::create_message, message_api::update_message,
            message_api::delete_message
        ]).mount("/auth", routes![auth::auth_user])
        .launch();
}