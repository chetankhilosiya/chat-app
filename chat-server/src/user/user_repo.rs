use crate::schema::users::dsl::*;
use diesel::prelude::*;
use super::user_model::{User, NewUser};
use crate::schema::users;

pub fn list(db: &MysqlConnection) -> QueryResult<Vec<User>> {
    users::table.load::<User>(db)
}

pub fn get(user_id: i32, db: &MysqlConnection) -> QueryResult<User> {
    users::table.find(user_id).get_result::<User>(db)
}

pub fn get_by_name(user_name: String, db: &MysqlConnection) -> QueryResult<User> {
    users.filter(name.eq(user_name)).first(db)
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

pub fn delete(user_id: i32, db: &MysqlConnection) -> QueryResult<usize> {
    diesel::delete(users::table.find(user_id))
        .execute(db)
}