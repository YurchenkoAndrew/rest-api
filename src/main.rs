use std::io::Result;
use dotenv::dotenv;
mod helpers;

use actix_web::{HttpServer, App, web::Data};
use posts::services::{get_posts, post_details, post_create, post_update, post_delete};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

mod users;
use users::services::{get_users, user_details, user_create, user_update, user_delete, get_user_post};

mod posts;

struct AppState {
    db: Pool<Postgres>
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let connecrion_string = std::env::var("DATABASE_URL").expect("Строка подлючения должна быть установлена!");
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connecrion_string)
        .await 
        .expect("Ошибка пула подключений!");
    sqlx::migrate!("./migrations").run(&pool).await.expect("Произошла ошибка миграции!");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState{db: pool.clone()}))
            .service(get_users)
            .service(user_details)
            .service(user_create)
            .service(user_update)
            .service(user_delete)
            .service(get_user_post)
            .service(get_posts)
            .service(post_details)
            .service(post_create)
            .service(post_update)
            .service(post_delete)
    }).bind(("127.0.0.1", 8080))?.run().await
}
