use actix_web::actix::Addr;
use crate::model::{db::ConnDsl, graphql::GraphQLExecutor};

pub struct AppState {
    pub db: Addr<ConnDsl>,
    pub gql_executor: Addr<GraphQLExecutor>,
}
