use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

use async_graphql::*;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{self, IntoResponse},
    routing::{get, post},
    Extension, Router,
};
use database::create_connection_pool;
use dotenvy::dotenv;
use resolvers::{MutationRoot, MySchema, QueryRoot};
use services::{auth::AuthService, project::ProjectService, wallet::WalletService};

mod database;
mod errors;
mod jwt;
mod resolvers;
pub mod schema;
mod services;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // database setup
    let database_connection = create_connection_pool();

    // services setup
    let project_service = ProjectService::new(database_connection.clone());
    let wallet_service = Arc::new(WalletService::new(database_connection.clone()));
    let auth_service = AuthService::new(wallet_service.clone());

    // schema setup
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(project_service)
        .data(wallet_service)
        .data(auth_service)
        .finish();

    // let allowed_origins = [
    //     "http://localhost:3001".parse().unwrap(),
    //     "http://localhost:3000".parse().unwrap(),
    // ];
    let cors = CorsLayer::permissive().expose_headers(Any);
    // axum setup
    let app = Router::new()
        .route("/", get(graphiql))
        .route("/graphql", post(graphql_handler))
        .layer(Extension(schema))
        .layer(cors);

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
    response::Html(http::GraphiQLSource::build().endpoint("/graphql").finish())
}
