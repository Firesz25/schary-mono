mod auth;
use actix_identity::Identity;
use actix_web::{error, web, HttpResponse};

use crate::state::State;
use tera::Context;
pub fn view(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index));
    cfg.route("/login", web::get().to(auth::login));
}

async fn index(state: web::Data<State>, id: Option<Identity>) -> actix_web::Result<HttpResponse> {
    let id = match id.map(|id| id.id()) {
        None => "anonymous".to_owned(),
        Some(Ok(id)) => id,
        Some(Err(err)) => return Err(error::ErrorInternalServerError(err)),
    };
    let tmpl = &state.tmpl;
    let mut ctx = Context::new();
    ctx.insert("user", &id);
    let html = tmpl.render("index.html.tera", &ctx).unwrap();
    Ok(HttpResponse::Ok().body(html))
}
