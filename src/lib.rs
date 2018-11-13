#![feature(custom_attribute)]
#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use]
extern crate juniper;
extern crate juniper_rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
extern crate rocket;
#[macro_use]
extern crate serde_json;
extern crate rocket_cors;
#[macro_use]
extern crate serde_derive;
extern crate frank_jwt;
extern crate grounded_path;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket_contrib;
extern crate uuid;

extern crate bcrypt;

extern crate walkdir;

use rocket::fairing::AdHoc;
use rocket::http::Method;
use rocket::Rocket;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

mod auth;
mod db;
mod filestore;
mod graphql;
mod schema;
mod server_error;
mod static_file;

use self::auth::Secret;

// #[route(OPTIONS, "/api/login")]
// fn options_login_handler<'a>() -> Response<'a> {
//     Response::build().finalize()
// }

embed_migrations!();

///Initialize the webserver
pub fn init_rocket() -> Rocket {
    let cors_options = rocket_cors::Cors {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![
            Method::Get,
            Method::Post,
            Method::Delete,
            Method::Put,
            Method::Patch,
        ]
        .into_iter()
        .map(From::from)
        .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: false,
        ..Default::default()
    };

    let pool = db::init_pool();

    match pool.get() {
        Ok(conn) => embedded_migrations::run(&*conn).expect("Migrations failed"),
        Err(_) => (),
    }

    let rocket: Rocket = rocket::ignite()
        .attach(AdHoc::on_attach(|rocket| {
            let secret_val: String = (*rocket.config().get_str("secret").unwrap_or("")).to_string();
            Ok(rocket.manage(Secret(secret_val)))
        }))
        .manage(pool.clone())
        .attach(cors_options.clone())
        .manage(cors_options)
        .manage(graphql::Schema::new(
            schema::QueryRoot {},
            schema::MutationRoot {},
        ))
        .mount("/", rocket_cors::catch_all_options_routes())
        .mount("/", static_file::routes())
        .mount("/api", graphql::routes())
        .mount("/api", auth::routes())
        // .mount("/", routes![options_login_handler])
        .catch(catchers![
            server_error::json_404,
            server_error::json_500,
            server_error::json_401,
            server_error::json_403,
        ]);

    rocket
}
