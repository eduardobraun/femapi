#![allow(proc_macro_derive_resolution_fallback)]
#![feature(custom_attribute)]
#![feature(plugin)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate actix_web;
extern crate bcrypt;
extern crate chrono;
extern crate dotenv;
// extern crate env_logger;
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
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_envlogger;
extern crate slog_stream;
extern crate slog_term;

use actix_web::{actix::System, server};
use slog::Drain;
use slog::Logger;
use slog::*;
use slog_atomic::*;
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex};

use std::sync::atomic::Ordering::SeqCst;

mod api;
mod filestore;
mod handler;
mod middleware;
mod model;
mod router;
mod share;

///Initialize the webserver
pub fn init_server() {
    ::std::env::set_var("RUST_LOG", "wapp=info");
    ::std::env::set_var("RUST_BACKTRACE", "1");
    // env_logger::init();
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let drain = AtomicSwitch::new(drain);
    let _ctrl = drain.ctrl();

    let root = Logger::root(
        drain.fuse(),
        o!("version" => env!("CARGO_PKG_VERSION"), "build-id" => env!("GIT_HASH")),
    );

    let addr = "0.0.0.0:4000".to_string();
    let log = root.new(o!("address" => addr.clone()));
    info!(log, "Starting web_service..";);
    debug!(log, "web_service"; "stage" => "start");
    let sys = System::new("wapp");

    let state_log = log.clone();
    server::new(move || {
        vec![
            router::app_state(state_log.clone()).boxed(),
            router::app().boxed(),
        ]
    })
    .bind(addr)
    .unwrap()
    .shutdown_timeout(2)
    .workers(2)
    .start();

    debug!(log, "web_service"; "stage" => "run");
    sys.run();
    debug!(log, "web_service"; "stage" => "stop");
    info!(log, "Exiting..";);
}
