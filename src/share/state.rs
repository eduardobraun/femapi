use actix_web::actix::Addr;
use crate::model::{db::Database, graphql::GraphQLExecutor};
use crate::share::common::Claims;
use slog::Logger;

pub struct AppState {
    pub logger: Logger,
    pub db: Addr<Database>,
    pub gql_executor: Addr<GraphQLExecutor>,
    pub claims: Option<Claims>,
}
