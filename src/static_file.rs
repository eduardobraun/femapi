use dotenv::dotenv;
use rocket::response::NamedFile;
use rocket::Route;
use std::env;
use std::path::{Path, PathBuf};

#[get("/<file..>", rank = 10)]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    // const WEB_DIRECTORY: &str = "../frontend/app/static";
    // info!("Getting file: {}", file.to_str().unwrap());

    match NamedFile::open(Path::new(&static_dir()).join(file.clone())) {
        Ok(file) => Some(file),
        Err(_) => {
            // if file.starts_with("api") {
            //     None
            // } else {
            // info!("Could not find file, returning index");
            Some(index())
            // }
        }
    }
}

#[get("/", rank = 10)]
pub fn index() -> NamedFile {
    // info!("Getting index.html");
    let INDEX_FILE = Path::new("index.html");
    NamedFile::open(Path::new(&static_dir()).join(INDEX_FILE)).unwrap()
}

pub fn routes() -> Vec<Route> {
    routes![files, index]
}

fn static_dir() -> String {
    dotenv().ok();
    env::var("STATIC_DIR").expect("STATIC_DIR must be set")
}
