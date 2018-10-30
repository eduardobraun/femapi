use dotenv::dotenv;
use rocket::response::NamedFile;
use rocket::Route;
use std::env;
use std::path::{Path, PathBuf};

#[get("/<file..>", rank = 10)]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    match NamedFile::open(Path::new(&static_dir()).join(file.clone())) {
        Ok(file) => Some(file),
        Err(_) => Some(index()),
    }
}

#[get("/", rank = 10)]
pub fn index() -> NamedFile {
    let index_file = Path::new("index.html");
    NamedFile::open(Path::new(&static_dir()).join(index_file)).unwrap()
}

pub fn routes() -> Vec<Route> {
    routes![files, index]
}

fn static_dir() -> String {
    dotenv().ok();
    env::var("STATIC_DIR").expect("STATIC_DIR must be set")
}
