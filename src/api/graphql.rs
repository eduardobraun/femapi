use actix_web::{
    AsyncResponder, FutureResponse, HttpMessage, HttpRequest, HttpResponse, Json, State,
};
use futures::Future;

use crate::model::graphql::GraphQLData;
use crate::share::state::AppState;

pub fn graphql((st, data): (State<AppState>, Json<GraphQLData>)) -> FutureResponse<HttpResponse> {
    st.gql_executor
        .send(data.0)
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}
