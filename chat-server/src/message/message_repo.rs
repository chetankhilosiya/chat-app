use diesel::prelude::*;
use super::message_model::{NewTextMessage, TextMessage};
use crate::schema::{messages, messages::dsl::*};

pub fn list_for_user(user_id: i32, db: &MysqlConnection) -> QueryResult<Vec<TextMessage>> {
    messages.filter(from_user.eq(user_id)).get_results::<TextMessage>(db)
}

pub fn list_to_user(user_id: i32, db: &MysqlConnection) -> QueryResult<Vec<TextMessage>> {
    messages.filter(to_user.eq(user_id)).get_results::<TextMessage>(db)
}

// pub fn get(message_id: i32, db: &MysqlConnection) -> QueryResult<TextMessage> {
//     messages::table.find(message_id).get_result::<TextMessage>(db)
// }

pub fn create(message: NewTextMessage, db: &MysqlConnection) -> QueryResult<usize> {
    diesel::insert_into(messages::table)
        .values(message)
        .execute(db)
}

pub fn update(message: TextMessage, db: &MysqlConnection) -> QueryResult<usize> {
    diesel::update(messages::table.find(message.id))
        .set(message)
        .execute(db)
}

pub fn delete(message_id: i32, db: &MysqlConnection) -> QueryResult<usize> {
    diesel::delete(messages::table.find(message_id))
        .execute(db)
}