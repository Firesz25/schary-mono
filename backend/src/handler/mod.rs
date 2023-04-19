mod auth;
use actix_web::web;

pub fn handler(cfg: &mut web::ServiceConfig) {
    cfg.route("/login", web::post().to(auth::login));
}
