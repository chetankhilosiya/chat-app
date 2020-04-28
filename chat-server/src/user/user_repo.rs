use diesel::prelude::*;
use super::user_model::{User, NewUser};
use crate::schema::users;

pub fn list(db: &MysqlConnection) -> QueryResult<Vec<User>> {
    users::table.load::<User>(db)
}

pub fn get(id: i32, db: &MysqlConnection) -> QueryResult<User> {
    users::table.find(id).get_result::<User>(db)
}

pub fn create(user: NewUser, db: &MysqlConnection) -> QueryResult<usize> {
    diesel::insert_into(users::table)
        .values(user)
        .execute(db)
}

pub fn update(user: User, db: &MysqlConnection) -> QueryResult<usize> {
    diesel::update(users::table.find(user.id))
        .set(user)
        .execute(db)
}

pub fn delete(id: i32, db: &MysqlConnection) -> QueryResult<usize> {
    diesel::delete(users::table.find(id))
        .execute(db)
}