#![allow(proc_macro_derive_resolution_fallback)]
#![feature(custom_attribute)]
#![feature(plugin)]

#[macro_use]
extern crate diesel;
extern crate diesel_migrations;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate actix_web;
extern crate bcrypt;
extern crate chrono;
extern crate dotenv;
extern crate env_logger;
extern crate futures;
extern crate http;
extern crate jsonwebtoken;
extern crate num_cpus;
extern crate postgres;
extern crate serde_json;
#[macro_use]
extern crate juniper;
// extern crate frank_jwt;
extern crate grounded_path;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate uuid;
extern crate walkdir;

use actix_web::{actix::System, server};

mod api;
mod handler;
mod middleware;
mod model;
mod router;
mod share;

///Initialize the webserver
pub fn init_server() {
    ::std::env::set_var("RUST_LOG", "wapp=info");
    ::std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let sys = System::new("wapp");

    server::new(move || vec![router::app_state().boxed(), router::app().boxed()])
        .bind("localhost:4000")
        .unwrap()
        .shutdown_timeout(2)
        .workers(2)
        .start();

    sys.run();
}
