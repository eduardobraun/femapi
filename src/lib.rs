#![feature(custom_attribute)]
#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use]
extern crate juniper;
extern crate juniper_rocket;
#[macro_use]
extern crate diesel;
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

extern crate bcrypt;

extern crate walkdir;

use rocket::fairing::AdHoc;
use rocket::http::Method;
use rocket::{Response, Rocket};
use rocket_cors::{AllowedHeaders, AllowedOrigins};

mod auth;
mod db;
mod filestore;
mod graphql;
mod schema;
mod server_error;
mod static_file;

use self::auth::Secret;

// #[options("/api/login")]
// fn login_options<'r>() -> impl Responder<'r> {
//     options.respond_owned(|guard| guard.responder(()))
// }
#[route(OPTIONS, "/api/login")]
fn options_login_handler<'a>() -> Response<'a> {
    Response::build().finalize()
}

///Initialize the webserver
pub fn init_rocket() -> Rocket {
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let connection = db::establish_connection();
    // let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    // let pool = r2d2::Pool::builder()
    //     .build(manager)
    //     .expect("Failed to create pool.");
    // let allowed_origins = AllowedOrigins::all();

    // response.set_header(Header::new(
    //     "Access-Control-Allow-Methods",
    //     "POST, GET, OPTIONS",
    // ));
    // response.set_header(Header::new(
    //     "Access-Control-Allow-Headers",
    //     "Content-Type, Authorization",
    // ));
    // response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    // You can also deserialize this
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
        // "Authorization",
        // "Accept",
        // "Content-Type",
        // "Accept-Encoding",
        // "Accept-Language",
        // "Connection",
        // "Cookie",
        // "Host",
        // "origin",
        // "User-Agent",
        // ]),
        allow_credentials: false,
        ..Default::default()
    };

    let pool = db::init_pool();

    let rocket: Rocket = rocket::ignite()
        .attach(AdHoc::on_attach(|rocket| {
            let secret_val: String = (*rocket.config().get_str("secret").unwrap_or("")).to_string();
            Ok(rocket.manage(Secret(secret_val)))
        }))
        // .manage(pool.clone().get())
        .manage(pool.clone())
        // .manage(Database::new())
        // .manage(Secret(
        //     rocket::config().get_string("secret_key").unwrap_or(-1),
        // ))
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
        .mount("/", routes![options_login_handler])
        .catch(catchers![
            server_error::json_404,
            server_error::json_500,
            server_error::json_401,
            server_error::json_403,
        ]);

    rocket
}
