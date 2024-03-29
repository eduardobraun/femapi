use dotenv::dotenv;
use grounded_path::GroundedPath;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use uuid::Uuid;
use walkdir::{DirEntry, WalkDir};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileNode {
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub extension: Option<String>,
    #[serde(default)]
    pub children: Option<Vec<FileNode>>,
    pub is_dir: bool,
}

pub struct FileStore;

fn is_dir(entry: &DirEntry) -> bool {
    entry.metadata().unwrap().is_dir()
}
fn is_file(entry: &DirEntry) -> bool {
    entry.metadata().unwrap().is_file()
}

fn get_filenode(path: GroundedPath) -> FileNode {
    if path
        .system_path()
        .metadata()
        .expect("metadata call failed")
        .is_dir()
    {
        return FileNode {
            name: path
                .local_path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            path: path.local_path().to_str().unwrap().to_string(),
            extension: None,
            children: Some(
                WalkDir::new(path.system_path())
                    .min_depth(1)
                    .max_depth(1)
                    .into_iter()
                    .map(|entry| {
                        let entry = entry.unwrap();
                        let path = GroundedPath::from_system(path.root_path(), entry.path())
                            .expect("Invalid sub path");
                        get_filenode(path)
                    })
                    .collect(),
            ),
            is_dir: true,
        };
    }

    FileNode {
        name: path
            .local_path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
        path: path.local_path().to_str().unwrap().to_string(),
        extension: Some(
            path.local_path()
                .extension()
                .unwrap_or(OsStr::new(""))
                .to_string_lossy()
                .to_string(),
        ),
        children: None,
        is_dir: false,
    }
}

impl FileStore {
    pub fn project_root(pid: Uuid) -> GroundedPath {
        dotenv().ok();
        GroundedPath::new(
            Path::new(&env::var("PROJECTS_DIR").expect("PROJECTS_DIR must be set"))
                .join(Path::new(&pid.to_string()))
                .as_path(),
        )
    }

    pub fn template_root(name: &str) -> GroundedPath {
        dotenv().ok();
        GroundedPath::new(
            Path::new(&env::var("TEMPLATES_DIR").expect("TEMPLATES_DIR must be set"))
                .join(&Path::new(name))
                .as_path(),
        )
    }

    pub fn dir(path: &GroundedPath) -> Vec<FileNode> {
        WalkDir::new(path.system_path())
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .map(|entry| {
                let entry = entry.unwrap();
                let path = GroundedPath::from_system(&path.system_path(), &entry.path())
                    .expect("Invalid sub path");
                get_filenode(path)
            })
            .collect::<Vec<FileNode>>()
    }

    pub fn create_all(path: &GroundedPath) -> Result<(), ()> {
        match std::fs::create_dir_all(path.system_path()) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    pub fn read(path: &GroundedPath) -> Result<String, ()> {
        let file = match File::open(path.system_path()) {
            Ok(f) => f,
            Err(_) => return Err(()),
        };
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).map_err(|_| ())?;
        Ok(contents)
    }

    pub fn copy_recursive(from: &GroundedPath, to: &GroundedPath) -> Result<(), ()> {
        for file in WalkDir::new(&from.system_path())
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_entry(|e| is_file(e))
        {
            let file = file.unwrap();
            match fs::copy(
                file.path(),
                to.clone()
                    .join(Path::new(file.file_name().to_str().unwrap()))
                    .system_path(),
            ) {
                Ok(_) => (),
                Err(_) => return Err(()),
            };
        }
        for dir in WalkDir::new(&from.system_path())
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_entry(|e| is_dir(e))
        {
            let dir = dir.unwrap();
            let to_dir = to
                .clone()
                .join(Path::new(dir.file_name().to_str().unwrap()));
            match Self::create_all(&to_dir) {
                Ok(_) => (),
                Err(_) => return Err(()),
            };
            match Self::copy_recursive(
                &from
                    .clone()
                    .join(Path::new(dir.file_name().to_str().unwrap())),
                &to_dir,
            ) {
                Ok(_) => (),
                Err(_) => return Err(()),
            };
        }
        Ok(())
    }
}
