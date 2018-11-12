#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::{members, projects, users};
use diesel::{Associations, Identifiable, Insertable, Queryable};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, PartialEq, Identifiable, Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: Uuid,
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Queryable, Identifiable, Clone, Debug)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub archived: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable)]
#[table_name = "projects"]
pub struct NewProject<'a> {
    pub id: Uuid,
    pub name: &'a str,
    pub archived: bool,
}

#[derive(Queryable, Clone, Associations, Debug)]
#[belongs_to(User, foreign_key = "user_id")]
#[belongs_to(Project, foreign_key = "project_id")]
pub struct Member {
    pub user_id: Uuid,
    pub project_id: Uuid,
    pub permission: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable)]
#[table_name = "members"]
pub struct NewMember<'a> {
    pub user_id: Uuid,
    pub project_id: Uuid,
    pub permission: &'a str,
}
