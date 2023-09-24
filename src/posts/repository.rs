use actix_web::web::{Data, Json};
use sqlx::Error;
use crate::{AppState, helpers::time_working::set_current_time};
use super::models::{PostForList, Post, PostRecord, PostEdited};

pub async fn get_posts(state: Data<AppState>) -> Result<Vec<PostForList>, Error> {
    let posts = sqlx::query_as::<_, PostForList>("SELECT id, title, description FROM posts").fetch_all(&state.db).await?;
    Ok(posts)
}

pub async fn post_details(state: Data<AppState>, id: i32) -> Result<Post, Error> {
    let post = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = $1").bind(id).fetch_one(&state.db).await;
    post
}

pub async fn post_create(state: Data<AppState>, id: i32, post: Json<PostRecord>) -> Result<Post, Error> {
    let post = sqlx::query_as::<_, Post>("INSERT INTO posts (title, description, created_at, updated_at, user_id) VALUES ($1, $2, $3, $4, $5) RETURNING id, title, description, created_at, updated_at, user_id")
    .bind(post.title.to_string())
    .bind(post.description.to_string())
    .bind(set_current_time())
    .bind(set_current_time())
    .bind(id)
    .fetch_one(&state.db).await;
    post
}

pub async fn post_update(state: Data<AppState>, id: i32, post: Json<PostEdited>) ->Result<Post, Error> {
    let post = sqlx::query_as::<_, Post>("UPDATE posts SET title = $2, description = $3, updated_at = $4, user_id = $5 WHERE id = $1 RETURNING id, title, description, created_at, updated_at, user_id")
    .bind(id)
    .bind(post.title.to_string())
    .bind(post.description.to_string())
    .bind(set_current_time())
    .bind(post.user_id)
    .fetch_one(&state.db)
    .await;
    post
}

pub async fn post_delete(state: Data<AppState>, id: i32) -> Result<Post, Error> {
    let post = sqlx::query_as::<_, Post>("DELETE FROM posts WHERE id = $1 RETURNING id, title, description, created_at, updated_at, user_id").bind(id).fetch_one(&state.db).await;
    post
}
