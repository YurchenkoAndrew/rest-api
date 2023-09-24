use actix_web::web::{Data, Json};
use sqlx::Error;
use crate::{helpers::time_working::set_current_time, posts::models::PostForList};
use super::models::{UserCreate, User, UserForList};
use crate::AppState;

pub async fn get_users(state: Data<AppState>) -> Result<Vec<UserForList>, Error> {
    let users = sqlx::query_as::<_,UserForList>("SELECT id, name, last_name, email FROM users ORDER BY id").fetch_all(&state.db).await?;
    Ok(users)
}

pub async fn user_details(state: Data<AppState>, id: i32) -> Result<User, Error> {
    let user = sqlx::query_as::<_,User>("SELECT id, name, last_name, email, created_at, updated_at FROM users WHERE id = $1").bind(id).fetch_one(&state.db).await;
    user
}

pub async fn create_user(state: Data<AppState>, new_user: Json<UserCreate>) -> Result<User, Error> {
    let result = sqlx::query_as::<_, User>("INSERT INTO users (name, last_name, email, created_at, updated_at) 
    VALUES ($1, $2, $3, $4, $5) RETURNING id, name, last_name, email, created_at, updated_at")
        .bind(new_user.name.to_string())
        .bind(new_user.last_name.to_string())
        .bind(new_user.email.to_string())
        .bind(set_current_time())
        .bind(set_current_time())
        .fetch_one(&state.db).await;
    result
}

pub async fn user_update(state: Data<AppState>, id: i32, user_update: Json<UserCreate>) -> Result<User, Error> {
    let user = sqlx::query_as::<_, User>("UPDATE users SET name = $1, last_name = $2, email = $3, updated_at = $4 WHERE id = $5 RETURNING id, name, last_name, email, created_at, updated_at")
    .bind(user_update.name.to_string())
    .bind(user_update.last_name.to_string())
    .bind(user_update.email.to_string())
    .bind(set_current_time())
    .bind(id)
    .fetch_one(&state.db)
    .await;
user
}

pub async fn user_delete(state: Data<AppState>, id: i32) -> Result<User, Error> {
    let user = sqlx::query_as::<_,User>("DELETE FROM users WHERE id = $1 RETURNING id, name, last_name, email, created_at, updated_at").bind(id).fetch_one(&state.db).await;
    user
}

pub async fn get_user_posts(state: Data<AppState>, id: i32) -> Result<Vec<PostForList>, Error> {
    let posts = sqlx::query_as::<_, PostForList>("SELECT id, title, description FROM posts WHERE user_id = $1").bind(id).fetch_all(&state.db).await;
    posts
}
