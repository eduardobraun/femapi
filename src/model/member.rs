use actix_web::actix::Message;
use crate::model::response::MyError;
use crate::model::{project::Project, user::User};
use crate::share::schema::members;
use diesel::{Associations, Identifiable, Insertable, Queryable};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Queryable, Identifiable, Clone, Associations, Debug, Deserialize, Serialize, Insertable,
)]
#[primary_key(user_id, project_id)]
#[belongs_to(User, foreign_key = "user_id")]
#[belongs_to(Project, foreign_key = "project_id")]
pub struct Member {
    pub user_id: Uuid,
    pub project_id: Uuid,
    pub permission: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddMember {
    pub user: User,
    pub project: Project,
    pub permission: String,
}

impl Message for AddMember {
    type Result = Result<Member, MyError>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectMembers {
    pub project: Project,
}

impl Message for ProjectMembers {
    type Result = Result<Vec<Member>, MyError>;
}

impl Member {
    pub fn new(project: &Project, user: &User) -> Member {
        Member {
            project_id: project.id.clone(),
            user_id: user.id.clone(),
            permission: "".to_string(),
        }
    }
}
