create table messages (
    id int primary key,
    from_user int not null,
    to_user int not null,
    text_message text not null
)