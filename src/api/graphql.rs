use actix_web::{AsyncResponder, FutureResponse, HttpRequest, HttpResponse, Json, State};
use futures::future::ok as FutOk;
use futures::Future;
use juniper::http::GraphQLRequest;

use crate::middleware::authenticator::RequestJWTAuth;
use crate::model::graphql::GraphQLData;
use crate::share::state::AppState;

pub fn graphql(
    (st, data, req): (State<AppState>, Json<GraphQLRequest>, HttpRequest<AppState>),
) -> FutureResponse<HttpResponse> {
    match req.claims() {
        Some(claims) => st
            .gql_executor
            .send(GraphQLData::new(data.0, claims))
            .from_err()
            .and_then(|res| match res {
                Ok(data) => Ok(HttpResponse::Ok()
                    .content_type("application/json")
                    .body(data)),
                Err(_) => Ok(HttpResponse::InternalServerError().into()),
            })
            .responder(),
        None => FutOk(HttpResponse::InternalServerError().into()).responder(),
    }
}
