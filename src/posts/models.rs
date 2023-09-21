use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Serialize, FromRow)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub user_id: i32
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct PostForList {
    id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub struct PostRecord {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize, FromRow, Serialize)]
pub struct PostEdited {
    pub title: String,
    pub description: String,
    pub user_id: i32
}