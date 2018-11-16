use actix_web::actix::{Actor, Addr, SyncArbiter, SyncContext};
use actix_web::{actix::Handler, actix::Message, Error};
use futures::Future;
use juniper::http::GraphQLRequest;

use crate::model::db::ConnDsl;
use crate::model::user::UserById;
use crate::share::common::Claims;
use crate::share::gql_schema::{create_schema, Schema, SchemaContext};

#[derive(Serialize, Deserialize)]
pub struct GraphQLData(GraphQLRequest, Claims);

impl GraphQLData {
    pub fn new(request: GraphQLRequest, claims: Claims) -> GraphQLData {
        GraphQLData(request, claims)
    }
}

impl Message for GraphQLData {
    type Result = Result<String, Error>;
}

pub struct GraphQLExecutor {
    schema: std::sync::Arc<Schema>,
    db_addr: Addr<ConnDsl>,
}

impl GraphQLExecutor {
    fn new(schema: std::sync::Arc<Schema>, db_addr: Addr<ConnDsl>) -> GraphQLExecutor {
        GraphQLExecutor {
            schema: schema,
            db_addr: db_addr,
        }
    }
}

impl Actor for GraphQLExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<GraphQLData> for GraphQLExecutor {
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: GraphQLData, _: &mut Self::Context) -> Self::Result {
        let user = match self
            .db_addr
            .send(UserById {
                user_id: msg.1.user_id.clone(),
            })
            .wait()
            .unwrap()
        {
            Ok(user) => Some(user),
            Err(_e) => None,
        };

        let res = msg.0.execute(
            &self.schema,
            &SchemaContext {
                current_user: user,
                db_addr: self.db_addr.clone(),
            },
        );
        let res_text = serde_json::to_string(&res)?;
        Ok(res_text)
    }
}

pub fn init(db_addr: Addr<ConnDsl>) -> Addr<GraphQLExecutor> {
    let schema = std::sync::Arc::new(create_schema());
    SyncArbiter::start(4, move || {
        GraphQLExecutor::new(schema.clone(), db_addr.clone())
    })
}
