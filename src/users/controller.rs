use actix_web::{Responder, web::{Path, Json, Data}, post, get, patch, delete, HttpResponse};
use crate::{AppState, users::repository::{self}};
use super::models::UserCreate;


#[get("/users")]
async fn get_users(state: Data<AppState>) -> impl Responder {
    match repository::get_users(state).await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::NotFound().json("Пользователи не найдены!"),
    }
}

#[get("/users/{id}/details")]
async fn user_details(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();
    match repository::user_details(state, id).await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().json("Пользователь с тами ID не найден!"),
    }
}

#[post("/users/create")]
async fn user_create(state: Data<AppState>, new_user: Json<UserCreate>) -> impl Responder {
    match repository::create_user(state, new_user).await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            eprintln!("Error creating user: {}", e);
            HttpResponse::InternalServerError().json("Internal Server Error")
        }
    }
}

#[patch("/users/{id}/update")]
async fn user_update(state: Data<AppState>, path: Path<i32>, user_update: Json<UserCreate>) -> impl Responder {
    let id: i32 = path.into_inner();
    match repository::user_update(state, id, user_update).await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::BadRequest().json("Произошла ошибка при обновлении данных пользователя!"),
    }
}

#[delete("/users/{id}")]
async fn user_delete(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();
    match repository::user_delete(state, id).await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().json("Произошла ошибка удаления пользователя"),
    }
}

#[get("/users/{id}/posts")]
async fn get_user_posts(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id = path.into_inner();
    match repository::get_user_posts(state, id).await
    {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(_) => HttpResponse::NotFound().json("У этого пользователя не найдено постов!")
    }
}
