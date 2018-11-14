use crate::model::{project::Project, user::User};

pub enum MyError {
    NotFound,
    DatabaseError,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Msgs {
    pub status: i32,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SigninMsgs {
    pub status: i32,
    pub token: String,
    pub signin_user: User,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProjectInfoMsgs {
    pub status: i32,
    pub project: Project,
    pub message: String,
}
