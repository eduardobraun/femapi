use actix_web::{actix::Message, Error};
use chrono::{NaiveDateTime, Utc};
use crate::model::response::MyError;
use crate::model::response::{Msgs, SigninMsgs};
use crate::share::schema::users;
use diesel::{Identifiable, Insertable, Queryable};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Identifiable, Queryable, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: Uuid,
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SignupUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SigninUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub user_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserUpdate {
    pub user_id: String,
    pub newname: String,
    pub newmail: String,
    pub newpassword: String,
    pub confirm_newpassword: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDelete {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserById {
    pub user_id: String,
}

impl Message for SignupUser {
    type Result = Result<Msgs, Error>;
}

impl Message for SigninUser {
    type Result = Result<SigninMsgs, Error>;
}

impl Message for UserById {
    type Result = Result<User, MyError>;
}

// impl Message for UserInfo {
//     type Result = Result<UserInfoMsgs, Error>;
// }

impl Message for UserUpdate {
    type Result = Result<Msgs, Error>;
}
impl Message for UserDelete {
    type Result = Result<Msgs, MyError>;
}

impl User {
    pub fn new() -> User {
        User {
            id: Uuid::new_v4(),
            email: "".to_string(),
            username: "".to_string(),
            password: "".to_string(),
            created_at: Utc::now().naive_utc(),
        }
    }
}
