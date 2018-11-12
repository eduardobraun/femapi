use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::response::Failure;
use rocket::Outcome;
use rocket::Route;
use rocket_contrib::Json;

use dotenv::dotenv;
use std::env;

use serde_derive::{Deserialize, Serialize};

use bcrypt::{hash, verify, DEFAULT_COST};
use frank_jwt::{decode, encode, Algorithm};
use uuid::Uuid;

use super::db::models::*;
use super::db::DbConn;
use super::diesel::prelude::*;

#[derive(Debug, Clone)]
pub struct Secret(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub struct AuthInfo {
    pub user_id: Uuid,
    pub user_email: String,
    pub user_name: String,
}

fn secret() -> String {
    dotenv().ok();
    env::var("SECRET").expect("SECRET must be set")
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthInfo {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AuthInfo, ()> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        let key: Vec<&str> = keys[0].split(' ').collect();
        if key.len() != 2 {
            return Outcome::Failure((Status::BadRequest, ()));
        }
        let key = key[1];
        match decode(&key.to_string(), &secret(), Algorithm::HS256) {
            Ok((_header, payload)) => Outcome::Success(AuthInfo {
                user_id: Uuid::parse_str(payload.get("user_id").unwrap().as_str().unwrap())
                    .unwrap(),
                user_email: payload
                    .get("user_email")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string(),
                user_name: payload
                    .get("user_name")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string(),
            }),
            Err(_e) => Outcome::Forward(()),
        }
    }
}

#[post("/login", data = "<login_request>")]
fn login(login_request: Json<LoginRequest>, conn: DbConn) -> Result<Json<TokenResponse>, Failure> {
    use super::db::schema::users::dsl::*;

    let login_info = login_request.into_inner();

    let results = users
        .filter(email.eq(login_info.email))
        .limit(1)
        .load::<User>(&*conn)
        .expect("Failed to query User");

    if results.len() == 0 {
        return Err(Failure(Status::Unauthorized));
    }

    let user = results.get(0).expect("Failed to get User");
    let valid = verify(&login_info.password, &user.password).expect("Failed to verify hash");

    if !valid {
        return Err(Failure(Status::Unauthorized));
    }

    let payload = json!({
        "user_id": user.id,
        "user_email": user.email,
        "user_name": user.name,
    });

    let header = json!({});
    let token =
        encode(header, &secret(), &payload, Algorithm::HS256).expect("Failed to encode token");
    let token_response = TokenResponse { token: token };

    Ok(Json(token_response))
}

#[post("/create_user", data = "<new_user_request>")]
fn new_user(
    new_user_request: Json<NewUserRequest>,
    conn: DbConn,
) -> Result<Json<&'static str>, Status> {
    use super::db::schema::users::dsl::*;

    let hashed = hash(&new_user_request.password, DEFAULT_COST).expect("Error hashing password");
    let new_user = NewUser {
        id: Uuid::new_v4(),
        name: &new_user_request.name.clone(),
        email: &new_user_request.email.clone(),
        password: &hashed.clone(),
    };

    diesel::insert_into(users)
        .values(new_user)
        .execute(&*conn)
        .expect("Failed to add user");

    Ok(Json("{'message': 'User added.'}"))
}

#[get("/renew")]
fn renew(auth_info: AuthInfo) -> Result<Json<TokenResponse>, Status> {
    let payload = json!({
        "user_id": auth_info.user_id,
        "user_email": auth_info.user_email,
        "user_name": auth_info.user_name,
    });

    let header = json!({});
    let token =
        encode(header, &secret(), &payload, Algorithm::HS256).expect("Failed to encode token");
    let token_response = TokenResponse { token: token };

    Ok(Json(token_response))
}

pub fn routes() -> Vec<Route> {
    routes![login, renew, new_user]
}
