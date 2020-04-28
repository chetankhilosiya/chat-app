use diesel::{Queryable, Insertable, AsChangeset };
use std::fmt::Debug;
use crate::schema::users;
use serde::{Serialize, Deserialize};

#[derive(Insertable, Queryable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name="users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub address: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="users"]
pub struct NewUser {
    pub name: String,
    pub address: Option<String>,
}