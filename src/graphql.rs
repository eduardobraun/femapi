use rocket::Route;

use super::db::Pool;
use juniper::RootNode;
use rocket::response::content;
use rocket::State;

use super::schema::{Database, MutationRoot, QueryRoot};

use super::db::DbConn;

use super::db::models::User;
use diesel::*;

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

#[get("/gql")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/api/graphql")
}

#[get("/graphql?<request>")]
fn get_graphql_handler(
    pool: State<Pool>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    use super::db::schema::users::dsl;
    let conn = DbConn(pool.clone().get().unwrap());
    let user = match dsl::users.order(dsl::id).first::<User>(&*conn) {
        Ok(u) => Some(u),
        _ => None,
    };
    let context = Database {
        current_user: user,
        pool: pool.clone(),
    };
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    pool: State<Pool>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    use super::db::schema::users::dsl;
    let conn = DbConn(pool.clone().get().unwrap());
    let user = match dsl::users.order(dsl::id).first::<User>(&*conn) {
        Ok(u) => Some(u),
        _ => None,
    };

    let context = Database {
        current_user: user,
        pool: pool.clone(),
    };
    request.execute(&schema, &context)
}

pub fn routes() -> Vec<Route> {
    routes![graphiql, get_graphql_handler, post_graphql_handler]
}
