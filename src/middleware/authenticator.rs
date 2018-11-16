use actix_web::{self, middleware::Started, HttpMessage, HttpRequest};
use crate::share::common::Claims;
use crate::share::state::AppState;
use jsonwebtoken::{decode, Validation};

pub struct Authenticator;

struct ClaimsBox(Box<Claims>);

pub trait RequestJWTAuth {
    fn claims(&self) -> Option<Claims>;
}

impl<S> RequestJWTAuth for HttpRequest<S> {
    fn claims(&self) -> Option<Claims> {
        if let Some(claims) = self.extensions().get::<ClaimsBox>() {
            return Some((*claims.0).clone());
        }
        None
    }
}

impl actix_web::middleware::Middleware<AppState> for Authenticator {
    fn start(&self, req: &HttpRequest<AppState>) -> actix_web::Result<Started> {
        match req.headers().get("Authorization") {
            Some(token) => {
                let token: Vec<&str> = token.to_str().unwrap().split(' ').collect();
                if token.len() != 2 {
                    return Err(actix_web::error::ErrorInternalServerError("Hmmmm"));
                }
                if token[0] != "Bearer" {
                    return Err(actix_web::error::ErrorInternalServerError("Hmm"));
                }
                let token = token[1];
                println!{"token: {:?}", token};
                let token = decode::<Claims>(&token, "secret".as_ref(), &Validation::default());
                println!{"decoded: {:?}", token};
                if let Err(_e) = token {
                    return Err(actix_web::error::ErrorInternalServerError("Hmm"));
                }
                let token = token.unwrap();
                req.extensions_mut()
                    .insert(ClaimsBox(Box::new(token.claims)));
                Ok(Started::Done)
            }
            None => Ok(Started::Done),
        }
    }
}
