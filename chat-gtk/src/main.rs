extern crate reqwest;
extern crate text_io;
extern crate chat_server;

mod server_api;

use text_io::read;
use std::io::Write;
use std::io;
use chat_server::user::user_model::User;

fn main() {
    println!("Welcome to chat app");
    let choice = get_login_choice();
    let result = match choice {
        1 => register_user(),
        2 => Some(get_user()),
        _ => {
            println!("Enter valid choice");
            None
        }
    };

    if result.is_none() {
        println!("Error login user. Exiting....Bye");
        return;
    }

    let logged_user = result.unwrap();
    println!("User {} successfully logged in", logged_user.name);
    println!("Enter the user name to chat with: ");
    let chatter_user = get_user();
    send_chat_message(logged_user.id, chatter_user.id);
    for message in server_api::get_messages(logged_user.id) {
        println!("From {}: {}", chatter_user.name, message.text_message);
    }
}

fn get_login_choice() -> u8 {
    println!("1. Register");
    println!("2. Login");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
    let choice: u8 = read!();
    println!("user input is : {}", choice);
    choice
}

fn register_user() -> Option<User> {
    print!("Enter the user name: ");
    io::stdout().flush().unwrap();
    let user_name: String = read!();
    print!("Enter the city address: ");
    io::stdout().flush().unwrap();
    let address: String = read!();
    if server_api::register_user(&user_name, &address) {
        Some(server_api::get_user(user_name))
    } else {
        None
    }
}

fn get_user() -> User {
    print!("Enter user name: ");
    io::stdout().flush().unwrap();
    let user_name: String = read!();
    server_api::get_user(user_name)
}

fn send_chat_message(from_user: i32, to_user: i32) {
    print!("Enter message: ");
    io::stdout().flush().unwrap();
    let mut message: String = String::new();
    io::stdin().read_line(&mut message).unwrap();
    server_api::send_message(from_user, to_user, message.trim().into());
}