table! {
    messages (id) {
        id -> Integer,
        from_user -> Integer,
        to_user -> Integer,
        text_message -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Text,
        address -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    messages,
    users,
);
