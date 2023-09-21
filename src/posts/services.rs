use actix_web::{get, post, patch, delete, web::{Data, Path, Json}, Responder, HttpResponse};

use crate::{AppState, helpers::time_working::set_current_time, posts::models::{PostEdited, PostForList}};
use super::models::{Post, PostRecord};

#[get("/posts")]
async fn get_posts(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, PostForList>("SELECT id, title, description FROM posts").fetch_all(&state.db).await 
    {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(_) => HttpResponse::NotFound().json("Постов не найдено!"),
    }
}

#[get("/posts/{id}")]
async fn post_details(state: Data<AppState>, path: Path<i32>) -> impl Responder 
{
    let id: i32 = path.into_inner();
    match sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = $1").bind(id).fetch_one(&state.db).await
    {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(_) => HttpResponse::NotFound().json("Пост по текущему идентификатору не найден!"),
    }
}

#[post("posts/{id}")]
async fn post_create(state: Data<AppState>, path: Path<i32>, post: Json<PostRecord>) -> impl Responder {
    let id: i32 = path.into_inner();
    // let current_time = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    match sqlx::query_as::<_, Post>("INSERT INTO posts (title, description, created_at, updated_at, user_id) VALUES ($1, $2, $3, $4, $5) RETURNING id, title, description, created_at, updated_at, user_id")
    .bind(post.title.to_string())
    .bind(post.description.to_string())
    .bind(set_current_time())
    .bind(set_current_time())
    .bind(id)
    .fetch_one(&state.db).await
    {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(e) => HttpResponse::InternalServerError().json(print!("Ошибка при создании поста! {}", e))
    }
}

#[patch("/posts/{id}")]
async fn post_update(state: Data<AppState>, path: Path<i32>, post: Json<PostEdited>) -> impl Responder {
    let id: i32 = path.into_inner();
    match sqlx::query_as::<_, Post>("UPDATE posts SET title = $2, description = $3, updated_at = $4, user_id = $5 WHERE id = $1 RETURNING id, title, description, created_at, updated_at, user_id")
        .bind(id)
        .bind(post.title.to_string())
        .bind(post.description.to_string())
        .bind(set_current_time())
        .bind(post.user_id)
        .fetch_one(&state.db)
        .await
    {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(e) => HttpResponse::InternalServerError().json(println!("Произошла ошибка при обновлении поста! {}", e)),
    }
}

#[delete("/posts/{id}")]
async fn post_delete(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();
    match sqlx::query_as::<_, Post>("DELETE FROM posts WHERE id = $1 RETURNING id, title, description, created_at, updated_at, user_id").bind(id).fetch_one(&state.db).await
    {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(_) => HttpResponse::NotFound().json("Пост с таким идентификатором был не найден!")
    }
}