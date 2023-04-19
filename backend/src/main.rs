use actix_identity::IdentityMiddleware;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use config::Config;
use state::State;

mod config;
mod entity;
mod handler;
mod model;
mod state;
mod view;
use actix_web::{
    cookie::{time::Duration, Key},
    middleware,
    web::Data,
    App, HttpServer,
};
// use tracing_actix_web::TracingLogger;
#[actix_web::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let cfg = Config::new();
    let state = State::new(cfg.clone()).await;
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::generate())
                    .cookie_name("auth-example".to_owned())
                    .cookie_secure(false)
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(Duration::minutes(10)),
                    )
                    .build(),
            )
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
            .configure(view::view)
            .configure(handler::handler)
    })
    .bind("[::1]:30000")
    .unwrap()
    .run()
    .await
    .unwrap();
}
