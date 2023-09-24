use actix_web::web;

use super::controller::{get_users, user_details, user_create, user_update, get_user_posts, user_delete};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_users)
        .service(user_details)
        .service(user_create)
        .service(user_update)
        .service(user_delete)
        .service(get_user_posts);
}
