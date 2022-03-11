use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum UserType {
    Normal,
    Admin,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub passwd: String,
    pub user_type: UserType,
}
