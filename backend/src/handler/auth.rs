use crate::{entity::prelude::*, model::auth::Login, state::State};
use actix_identity::Identity;
use actix_web::{http::header::LOCATION, web, HttpMessage, HttpRequest, HttpResponse};

pub async fn login(
    req: HttpRequest,
    form: web::Form<Login>,
    state: web::Data<State>,
) -> HttpResponse {
    let form = form.into_inner();
    let conn = &state.db;
    let user = match User::find_by_login(form.login, conn).await {
        Some(ok) => ok,
        None => return HttpResponse::Unauthorized().body(""),
    };
    if user.password == form.password {
        Identity::login(&req.extensions(), user.login).unwrap();
        HttpResponse::Found()
            .append_header((LOCATION, "/"))
            .finish()
    } else {
        HttpResponse::Unauthorized().body("")
    }
}
