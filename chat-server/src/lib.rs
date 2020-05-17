#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

pub mod user;
pub mod message;
pub mod auth;
pub mod schema;

use rocket::{Request, Outcome, State};
use rocket::request::{self, FromRequest};
use r2d2_diesel::ConnectionManager;
use diesel::prelude::*;
use rocket::http::Status;
use std::ops::Deref;

type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
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
