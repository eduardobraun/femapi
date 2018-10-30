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

extern crate bcrypt;

use std::path::{Path, PathBuf};

use juniper::EmptyMutation;
use rocket::fairing::{AdHoc, Fairing, Info, Kind};
use rocket::http::{ContentType, Header, Method};
use rocket::{Request, Response, Rocket};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Guard, Responder};
use std::io::Cursor;

mod db;

// mod model;
mod schema;

// use self::model::Database;

mod auth;
mod graphql;
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
        // .attach(CORS())
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
        // .manage(db_pool)
        // .manage(secret)
        // .manage(banned_set)
        .mount("/", static_file::routes())
        .mount("/api", graphql::routes())
        .mount("/api", auth::routes())
        .mount("/", routes![options_login_handler])
        // .mount(&format_api(Forum::PATH), Forum::ROUTES())
        // .mount(&format_api(Thread::PATH), Thread::ROUTES())
        // .mount(&format_api(Post::PATH), Post::ROUTES())
        // .mount(&format_api(Bucket::PATH), Bucket::ROUTES())
        // .mount(&format_api(Question::PATH), Question::ROUTES())
        // .mount(&format_api(Answer::PATH), Answer::ROUTES())
        // .mount(&format_api(Chat::PATH), Chat::ROUTES())
        // .mount(&format_api(Message::PATH), Message::ROUTES())
        .catch(catchers![
            server_error::json_404,
            server_error::json_500,
            server_error::json_401,
            server_error::json_403,
        ]);

    rocket
}

pub struct CORS();

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        if request.method() == Method::Options || response.content_type() == Some(ContentType::JSON)
        {
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, GET, OPTIONS",
            ));
            response.set_header(Header::new(
                "Access-Control-Allow-Headers",
                "Content-Type, Authorization",
            ));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        }

        if request.method() == Method::Options {
            response.set_header(ContentType::Plain);
            response.set_sized_body(Cursor::new(""));
        }
    }
}
