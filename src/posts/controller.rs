use actix_web::{get, post, patch, delete, web::{Data, Path, Json}, Responder, HttpResponse};
use crate::{AppState, posts::{models::PostEdited, repository}};
use super::models::PostRecord;

#[get("/posts")]
async fn get_posts(state: Data<AppState>) -> impl Responder {
    match repository::get_posts(state).await
    {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(_) => HttpResponse::NotFound().json("Постов не найдено!"),
    }
}

#[get("/posts/{id}")]
async fn post_details(state: Data<AppState>, path: Path<i32>) -> impl Responder 
{
    let id: i32 = path.into_inner();
    match repository::post_details(state, id).await
    {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(_) => HttpResponse::NotFound().json("Пост по текущему идентификатору не найден!"),
    }
}

#[post("posts/{id}")]
async fn post_create(state: Data<AppState>, path: Path<i32>, post: Json<PostRecord>) -> impl Responder {
    let id: i32 = path.into_inner();
    match repository::post_create(state, id, post).await
    {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(e) => HttpResponse::InternalServerError().json(print!("Ошибка при создании поста! {}", e))
    }
}

#[patch("/posts/{id}")]
async fn post_update(state: Data<AppState>, path: Path<i32>, post: Json<PostEdited>) -> impl Responder {
    let id: i32 = path.into_inner();
    match repository::post_update(state, id, post).await
    {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(e) => HttpResponse::InternalServerError().json(println!("Произошла ошибка при обновлении поста! {}", e)),
    }
}

#[delete("/posts/{id}")]
async fn post_delete(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();
    match repository::post_delete(state, id).await
    {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(_) => HttpResponse::NotFound().json("Пост с таким идентификатором был не найден!")
    }
}