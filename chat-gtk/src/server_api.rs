use reqwest::blocking::Client;
use chat_server::user::user_model::{User, NewUser};
use chat_server::message::message_model::{TextMessage, NewTextMessage};

pub fn get_user(user_name: String) -> User {
    let client = reqwest::blocking::Client::new();
    let mut url: String = String::from("http://localhost:8000/users/name/");
    url.push_str(&user_name);
    let res: User = client.get(&url)
        .send().expect(&format!("Error getting user with name: {}", user_name))
        .json::<User>().expect("Error parsing user object");
    res
}

pub fn register_user(user_name: &str, address: &str) -> bool {
    let new_user = NewUser {
        name: String::from(user_name),
        address: Some(String::from(address)),
    };
    
    let client: Client = reqwest::blocking::Client::new();
    let url = "http://localhost:8000/users";
    let res: usize = client.post(url)
        .json(&new_user)
        .send().expect("error while registering user")
        .json::<usize>().expect("Error parsing resonse while creating user");
    
    res == 1
}

pub fn send_message(from_user: i32, to_user: i32, message: String) -> usize {
    let new_msg = NewTextMessage {
        from_user, to_user, text_message: message,
    };

    let client: Client = reqwest::blocking::Client::new();
    let url = "http://localhost:8000/messages/";
    let res: usize = client.post(url)
        .json(&new_msg)
        .send().expect("Error while sending message")
        .json::<usize>().expect("Error parsing send message response");
    res
}

pub fn get_messages(user_id: i32) -> Vec<TextMessage> {
    let mut url = String::from("http://localhost:8000/messages/");
    url.push_str(&user_id.to_string());
    let client: Client = reqwest::blocking::Client::new();
    let res = client.get(&url)
        .send().expect("Error getting messages")
        .json::<Vec<TextMessage>>().expect("Error parsing messages object");
    res
}