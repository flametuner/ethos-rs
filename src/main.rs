use std::net::SocketAddr;

use async_graphql::*;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Extension, Router,
};
use database::{create_connection_pool, ConnectionPool};
use graphql::{MutationRoot, MySchema, QueryRoot};
use services::project::ProjectService;

mod database;
mod errors;
mod graphql;
pub mod schema;
mod services;

#[tokio::main]
async fn main() {
    // database setup
    let database_connection = create_connection_pool();
    let connection_pool = ConnectionPool::new(database_connection.clone());

    // services setup
    let project_service = ProjectService::new(connection_pool);

    // schema setup
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(project_service)
        .finish();

    // axum setup
    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema));

    // lift off
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn graphql_handler(schema: Extension<MySchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
    response::Html(http::GraphiQLSource::build().endpoint("/").finish())
}
