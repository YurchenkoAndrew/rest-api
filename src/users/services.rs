use actix_web::{Responder, web::{Path, Json, Data}, post, get, patch, delete, HttpResponse};
use chrono::Utc;

use crate::{AppState, users::models::UserForList};

use super::models::{User, UserCreate};


#[get("/users")]
async fn get_users(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_,UserForList>("SELECT name, last_name, email FROM users").fetch_all(&state.db).await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::NotFound().json("Пользователи не найдены!"),
    }
}

#[get("/users/{id}/details")]
async fn user_details(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();
    match sqlx::query_as::<_,User>("SELECT id, name, last_name, email, created_at, updated_at FROM users WHERE id = $1").bind(id).fetch_one(&state.db).await 
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().json("Пользователь с тами ID не найден!"),
    }
}

#[post("/users/create")]
async fn user_create(state: Data<AppState>, new_user: Json<UserCreate>) -> impl Responder {
    let current_time = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    match sqlx::query_as::<_, User>("INSERT INTO users (name, last_name, email, created_at, updated_at) 
    VALUES ($1, $2, $3, $4, $5) RETURNING id, name, last_name, email, created_at, updated_at")
        .bind(new_user.name.to_string())
        .bind(new_user.last_name.to_string())
        .bind(new_user.email.to_string())
        .bind(&current_time)
        .bind(&current_time)
        .fetch_one(&state.db)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(println!("{}", e)),
    }
}

#[patch("/users/{id}/update")]
async fn user_update(state: Data<AppState>, path: Path<i32>, user_update: Json<UserCreate>) -> impl Responder {
    let id: i32 = path.into_inner();
    let current_time = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    match sqlx::query_as::<_, User>("UPDATE users SET name = $1, last_name = $2, email = $3, updated_at = $4 WHERE id = $5 RETURNING id, name, last_name, email, created_at, updated_at")
    .bind(user_update.name.to_string())
    .bind(user_update.last_name.to_string())
    .bind(user_update.email.to_string())
    .bind(current_time)
    .bind(id)
    .fetch_one(&state.db)
    .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::BadRequest().json("Произошла ошибка при обновлении данных пользователя!"),
    }
}

#[delete("/users/{id}")]
async fn user_delete(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();
    match sqlx::query_as::<_,User>("DELETE FROM users WHERE id = $1 RETURNING id, name, last_name, email, created_at, updated_at").bind(id).fetch_one(&state.db).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().json("Произошла ошибка удаления пользователя"),
    }
}