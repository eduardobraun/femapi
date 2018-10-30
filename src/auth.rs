use rocket::http::Status;
use rocket::response::Failure;
use rocket::Route;
use rocket::State;
use rocket_contrib::Json;

use serde_derive::{Deserialize, Serialize};

use bcrypt::{hash, verify, DEFAULT_COST};
use frank_jwt::{encode, Algorithm};

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

#[post("/login", data = "<login_request>")]
fn login(
    login_request: Json<LoginRequest>,
    secret: State<Secret>,
    conn: DbConn,
) -> Result<Json<TokenResponse>, Failure> {
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
    let token = encode(header, &secret.inner().0, &payload, Algorithm::HS256)
        .expect("Failed to encode token");
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
fn renew(secret: State<Secret>) -> Result<Json<&'static str>, Status> {
    println!{"The secret is: {:?}", secret};
    Ok(Json("{'token': 'token'}"))
}

pub fn routes() -> Vec<Route> {
    routes![login, renew, new_user]
}
