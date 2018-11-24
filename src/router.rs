use actix_web::{
    fs,
    http::{header, Method},
    middleware::{self, cors::Cors},
    App,
};
use slog::Logger;

use crate::model::db;
use crate::model::graphql;
use crate::share::state::AppState;

use crate::api::{
    auth::{signin, signup},
    graphql::graphql as gql_handler,
    home::{index, path, static_dir},
};

use crate::middleware::authenticator::Authenticator;
use crate::middleware::request_logger::RequestLogger;

pub fn app_state(logger: Logger) -> App<AppState> {
    let db_addr = db::init(logger.clone());
    let graphql_addr = graphql::init(db_addr.clone());
    App::with_state(AppState {
        logger: logger,
        db: db_addr.clone(),
        gql_executor: graphql_addr,
        claims: None,
    })
    .middleware(RequestLogger {})
    .middleware(Authenticator {})
    .prefix("/api")
    .configure(|app| {
        Cors::for_app(app)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::ORIGIN,
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600)
            .resource("/signup", |r| {
                r.method(Method::POST).with(signup);
            })
            .resource("/signin", |r| {
                r.method(Method::POST).with(signin);
            })
            .resource("/graphql", |r| {
                r.method(Method::POST).with(gql_handler);
            })
            .register()
    })
}

pub fn app() -> App {
    App::new()
        .middleware(middleware::Logger::default())
        .resource("/", |r| r.f(index))
        .resource("/a/{tail:.*}", |r| r.f(path))
        .handler("/", fs::StaticFiles::new(static_dir()).unwrap())
}
