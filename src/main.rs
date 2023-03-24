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
    println!("Starting Graphql WebApp...");
    use std::time::Instant;
    let now = Instant::now();
    // load environment variables
    dotenv().ok();
    // database setup
    let database_time = Instant::now();
    println!("Connecting to database...");
    let database_connection = create_connection_pool();

    println!(
        "Connected to database! ({}ms)",
        database_time.elapsed().as_millis()
    );
    // services setup
    println!("Setting up services...");
    let project_service = ProjectService::new(database_connection.clone());
    let wallet_service = Arc::new(WalletService::new(database_connection.clone()));
    let auth_service = AuthService::new(wallet_service.clone());

    // schema setup
    println!("Setting up schema...");
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(project_service)
        .data(wallet_service)
        .data(auth_service)
        .finish();

    // cors setup
    // let allowed_origins = [
    //     "http://localhost:3001".parse().unwrap(),
    //     "http://localhost:3000".parse().unwrap(),
    // ];
    println!("Setting up cors...");
    let cors = CorsLayer::permissive().expose_headers(Any);
    // axum setup
    let app = Router::new()
        .route("/", get(graphiql))
        .route("/graphql", post(graphql_handler))
        .layer(Extension(schema))
        .layer(cors);

    // liftoff
    println!("Liftoff in {}ms", now.elapsed().as_millis());
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
