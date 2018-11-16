use actix_web::{actix::Handler, error, Error};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use diesel::{self, ExpressionMethods, QueryDsl, RunQueryDsl};
use jsonwebtoken::{encode, Header};
use uuid::Uuid;

use crate::model::db::ConnDsl;
use crate::model::response::MyError;
use crate::model::response::{Msgs, SigninMsgs};
use crate::model::user::{NewUser, SigninUser, SignupUser, User, UserById};
use crate::share::common::Claims;

impl Handler<UserById> for ConnDsl {
    type Result = Result<User, MyError>;

    fn handle(&mut self, user_by_id: UserById, _: &mut Self::Context) -> Self::Result {
        match Uuid::parse_str(&user_by_id.user_id) {
            Ok(id) => {
                use crate::share::schema::users::dsl;
                let conn = &self.0.get().map_err(|_| MyError::DatabaseError)?;
                Ok(dsl::users
                    .find(id)
                    .first::<User>(conn)
                    .map_err(|_| MyError::NotFound)?)
            }
            Err(_e) => Err(MyError::NotFound),
        }
    }
}

impl Handler<SignupUser> for ConnDsl {
    type Result = Result<Msgs, Error>;

    fn handle(&mut self, signup_user: SignupUser, _: &mut Self::Context) -> Self::Result {
        if &signup_user.password == &signup_user.confirm_password {
            use crate::share::schema::users::dsl::*;
            let hash_password = match hash(&signup_user.password, DEFAULT_COST) {
                Ok(h) => h,
                Err(_) => panic!(),
            };
            let new_user = NewUser {
                id: Uuid::new_v4(),
                email: &signup_user.email,
                username: &signup_user.username,
                password: &hash_password,
                created_at: Utc::now().naive_utc(),
            };
            let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
            diesel::insert_into(users)
                .values(&new_user)
                .execute(conn)
                .map_err(error::ErrorInternalServerError)?;
            Ok(Msgs {
                status: 200,
                message: "Successful Signup.".to_string(),
            })
        } else {
            Ok(Msgs {
                status: 400,
                message: "failed Signup.".to_string(),
            })
        }
    }
}

impl Handler<SigninUser> for ConnDsl {
    type Result = Result<SigninMsgs, Error>;

    fn handle(&mut self, signin_user: SigninUser, _: &mut Self::Context) -> Self::Result {
        use crate::share::schema::users::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let login_user = users
            .filter(&username.eq(&signin_user.username))
            .load::<User>(conn)
            .map_err(error::ErrorInternalServerError)?
            .pop();
        let no_user = User::new();
        match login_user {
            Some(login_user) => {
                match verify(&signin_user.password, &login_user.password) {
                    Ok(_valid) => {
                        let key = "secret";
                        let now = Utc::now().naive_utc().timestamp();
                        let until = now + 10800;
                        let claims = Claims {
                            user_id: login_user.id.to_string(),
                            iat: now,
                            exp: until,
                        };
                        let token = match encode(&Header::default(), &claims, key.as_ref()) {
                            Ok(t) => t,
                            Err(_) => panic!(), // in practice you would return the error
                        };
                        let the_user = User {
                            id: login_user.id,
                            email: login_user.email.clone(),
                            username: login_user.username.clone(),
                            password: login_user.password.clone(),
                            created_at: login_user.created_at.clone(),
                        };
                        Ok(SigninMsgs {
                            status: 200,
                            token: token,
                            signin_user: the_user,
                            message: "Succesfully signin.".to_string(),
                        })
                    }
                    Err(_) => Ok(SigninMsgs {
                        status: 400,
                        token: "".to_owned(),
                        signin_user: no_user,
                        message: "Incorrect Password.".to_string(),
                    }),
                }
            }
            None => Ok(SigninMsgs {
                status: 400,
                token: "".to_owned(),
                signin_user: no_user,
                message: "Signin failure.".to_string(),
            }),
        }
    }
}
