use std::io::Result;
use dotenv::dotenv;

use actix_web::{HttpServer, App, web::Data};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

mod users;
use users::services::{get_users, user_details, user_create, user_update, user_delete};

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
    }).bind(("127.0.0.1", 8080))?.run().await
}