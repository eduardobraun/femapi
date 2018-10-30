use rocket::response::NamedFile;
use rocket::Route;
use std::path::{Path, PathBuf};

#[get("/<file..>", rank = 10)]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    const WEB_DIRECTORY: &str = "../frontend/app/static";
    // info!("Getting file: {}", file.to_str().unwrap());

    match NamedFile::open(Path::new(WEB_DIRECTORY).join(file.clone())) {
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
    const WEB_DIRECTORY: &str = "../../frontend/app/static/index.html";
    NamedFile::open(Path::new(WEB_DIRECTORY)).unwrap()
}

pub fn routes() -> Vec<Route> {
    routes![files, index]
}
