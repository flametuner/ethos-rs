use std::net::SocketAddr;

use async_graphql::*;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Extension, Router,
};
use database::create_connection_pool;
use resolvers::{MutationRoot, MySchema, QueryRoot};
use services::{project::ProjectService, wallet::WalletService};

mod database;
mod errors;
mod resolvers;
pub mod schema;
mod services;

#[tokio::main]
async fn main() {
    // database setup
    let database_connection = create_connection_pool();

    // services setup
    let project_service = ProjectService::new(database_connection.clone());
    let wallet_service = WalletService::new(database_connection.clone());

    // schema setup
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(project_service)
        .data(wallet_service)
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
