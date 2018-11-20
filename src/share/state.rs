use actix_web::actix::Addr;
use crate::model::{db::ConnDsl, graphql::GraphQLExecutor};
use crate::share::common::Claims;

pub struct AppState {
    pub db: Addr<ConnDsl>,
    pub gql_executor: Addr<GraphQLExecutor>,
    pub claims: Option<Claims>,
}
