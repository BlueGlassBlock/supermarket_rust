mod model;

pub use model::{User, UserType};
use std::fs;
use std::io;

fn main() {
    let separator: String =
        "==========================================================".to_string();
    println!("Welcome to SupermarketRust!");
    println!("{}", separator);
    match fs::read_to_string("admin.json") {
        Ok(admin_str) => {
            let admin: User = serde_json::from_str(&admin_str).unwrap();
            println!("admin: {}", admin.name);
        }
        Err(_) => {
            println!("admin.json not found");
            println!("Creating admin account...");
            println!("Please enter your password:");
            let mut pwd: String = String::new();
            io::stdin().read_line(&mut pwd).unwrap();
            let admin = User {
                name: "admin".to_string(),
                passwd: pwd.trim_end().to_string(),
                user_type: UserType::Admin,
            };
            fs::write("admin.json", serde_json::to_vec(&admin).unwrap()).unwrap();
        }
    };
    println!("Please select from following:");
    println!("1. Login");
    println!("2. Signup");
    println!("3. Exit");
    println!("{}", separator);
    let mut choice: String = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let mut users: Vec<User> = Vec::new();
    match fs::read_to_string("user.json") {
        Ok(user_table_str) => {
            users = serde_json::from_str(&user_table_str).unwrap();
        }
        Err(_) => {
            println!("user.json not found");
            println!("Creating user table...");
            let mut user_table_str: String = String::new();
            user_table_str.push_str("[\n");
            user_table_str.push_str("]\n");
            fs::write("user.json", user_table_str).unwrap();
        }
    };

    match choice.trim_end().parse::<i32>() {
        Ok(1) => {
            println!("Please enter your name:");
            let mut name: String = String::new();
            io::stdin().read_line(&mut name).unwrap();
            println!("Please enter your password:");
            let mut pwd: String = String::new();
            io::stdin().read_line(&mut pwd).unwrap();
            let _user = User {
                name: name.trim_end().to_string(),
                passwd: pwd.trim_end().to_string(),
                user_type: UserType::Normal,
            };
            //TODO: check if user exists
        }
        Ok(2) => {
            println!("Please enter your name:");
            let mut name: String = String::new();
            io::stdin().read_line(&mut name).unwrap();
            println!("Please enter your password:");
            let mut pwd: String = String::new();
            io::stdin().read_line(&mut pwd).unwrap();
            let user = User {
                name: name.trim_end().to_string(),
                passwd: pwd.trim_end().to_string(),
                user_type: UserType::Normal,
            };
            let name = user.name.clone();
            users.push(user);
            fs::write("user.json", serde_json::to_vec(&users).unwrap()).unwrap();
            println!("Successfully created user {}", name);
            // TODO: check conflict
            // TODO: convert to hashmap
            return;
        }
        Ok(3) => {
            println!("Bye!");
            return;
        }
        _ => println!("Invalid choice!"),
    }
}
