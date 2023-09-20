use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct UserCreate {
    pub name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Serialize, FromRow)]
pub struct UserForList {
    pub name: String,
    pub last_name: String,
    pub email: String,
}