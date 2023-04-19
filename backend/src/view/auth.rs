use crate::state::State;
use actix_web::{web, HttpResponse};
use tera::Context;

pub async fn login(state: web::Data<State>) -> HttpResponse {
    let tmpl = &state.tmpl;
    let html = tmpl.render("login.html.tera", &Context::new()).unwrap();
    HttpResponse::Ok().body(html)
}
