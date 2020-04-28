use diesel::{Queryable, Insertable, AsChangeset };
use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use crate::schema::messages;

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name="messages"]
pub struct TextMessage {
    pub id: i32,
    pub from_user: i32,
    pub to_user: i32,
    pub text_message: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="messages"]
pub struct NewTextMessage {
    pub from_user: i32,
    pub to_user: i32,
    pub text_message: String,
}
