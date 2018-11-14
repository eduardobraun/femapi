use actix_web::{fs::NamedFile, Error, HttpRequest, Result};
use dotenv::dotenv;
use std::env;
use std::path::Path;

pub fn index(_req: &HttpRequest) -> Result<NamedFile> {
    let index_file = Path::new("index.html");
    Ok(NamedFile::open(Path::new(&static_dir()).join(index_file))?)
}

pub fn path(req: &HttpRequest) -> Result<NamedFile> {
    index(req)
}

pub fn static_dir() -> String {
    dotenv().ok();
    env::var("STATIC_DIR").expect("STATIC_DIR must be set")
}
