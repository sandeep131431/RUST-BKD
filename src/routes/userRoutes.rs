use actix_web::web;
use crate::handler::{index, create_user, get_users, login_user};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        // Root URL
        .route("/", web::get().to(index))
        // API routes
        .route("/user", web::post().to(create_user))
        .route("/login", web::post().to(login_user)) // ✅ Login route add kiya
        .route("/users", web::get().to(get_users));
}