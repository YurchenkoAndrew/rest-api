use actix_web::web;

use super::controller::{get_posts, post_details, post_create, post_update, post_delete};


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
    .service(get_posts)
    .service(post_details)
    .service(post_create)
    .service(post_update)
    .service(post_delete);
}