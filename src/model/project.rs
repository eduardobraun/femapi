use actix_web::actix::Message;
use chrono::{NaiveDateTime, Utc};
use crate::model::response::MyError;
use crate::model::user::User;
use crate::share::schema::projects;
use diesel::{Identifiable, Insertable, Queryable};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Queryable, Identifiable, Clone, Debug)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub archived: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable)]
#[table_name = "projects"]
pub struct NewProject<'a> {
    pub id: Uuid,
    pub name: &'a str,
    pub archived: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateProject {
    pub name: String,
    pub user: User,
}

impl Message for CreateProject {
    type Result = Result<Project, MyError>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectById {
    pub project_id: String,
}

impl Message for ProjectById {
    type Result = Result<Project, MyError>;
}

impl Project {
    pub fn new() -> Project {
        Project {
            id: Uuid::new_v4(),
            name: "".to_string(),
            archived: false,
            created_at: Utc::now().naive_utc(),
        }
    }
}
