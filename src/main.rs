use std::{net::SocketAddr, str::FromStr, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

use async_graphql::*;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    http::HeaderMap,
    response::{self, IntoResponse},
    routing::{get, post},
    Extension, Router,
};
use dotenvy::dotenv;
use uuid::Uuid;

use ethos_rs::database::{create_connection_pool, ConnectionPool};
use ethos_rs::resolvers::{MutationRoot, QueryRoot};
use ethos_rs::services::{
    auth::AuthService, nft::NftService, profile::ProfileService, project::ProjectService,
    wallet::WalletService,
};

struct AppState {
    auth_service: Arc<AuthService>,
    project_service: Arc<ProjectService>,
}

pub type MySchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

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
    let project_service = Arc::new(ProjectService::new(database_connection.clone()));
    let profile_service = ProfileService::new(database_connection.clone());
    let pool = ConnectionPool::new(database_connection.clone());
    let wallet_service = Arc::new(WalletService::new(pool));
    let auth_service = Arc::new(AuthService::new(wallet_service.clone()));
    let collection_service = NftService::new(database_connection.clone());
    let nft_service = NftService::new(database_connection.clone());

    // schema setup
    println!("Setting up schema...");
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(project_service.clone())
        .data(wallet_service)
        .data(auth_service.clone())
        .data(profile_service)
        .data(nft_service)
        .data(collection_service)
        .finish();

    // cors setup
    // let allowed_origins = [
    //     "http://localhost:3001".parse().unwrap(),
    //     "http://localhost:3000".parse().unwrap(),
    // ];
    println!("Setting up cors...");
    let cors = CorsLayer::permissive().expose_headers(Any);

    let state = Arc::new(AppState {
        auth_service,
        project_service,
    });

    // axum setup
    let app = Router::new()
        .route("/", get(graphiql))
        .route("/graphql", post(graphql_handler))
        .layer(Extension(schema))
        .layer(cors)
        .with_state(state);

    // liftoff
    println!("Liftoff in {}ms", now.elapsed().as_millis());
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn get_token_from_headers(headers: &HeaderMap) -> Option<String> {
    headers.get("Authorization").and_then(|value| {
        value.to_str().ok().map(|s| {
            let s = s.to_string();
            if s.to_lowercase().contains("bearer ") {
                return s[7..].to_string();
            }
            s
        })
    })
}

fn get_project_from_headers(headers: &HeaderMap) -> Option<String> {
    headers
        .get("project")
        .and_then(|value| value.to_str().ok().map(|v| v.to_string()))
}

async fn graphql_handler(
    schema: Extension<MySchema>,
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let auth = &state.auth_service;
    let project_service = &state.project_service;

    let mut req = req.into_inner();
    if let Some(token) = get_token_from_headers(&headers) {
        if let Ok(wallet) = auth.validate(token.as_str()).await {
            req = req.data(wallet);
        }
    }
    if let Some(project_id) = get_project_from_headers(&headers) {
        if let Ok(id) = Uuid::from_str(&project_id) {
            if let Ok(project) = project_service.get_project(id) {
                req = req.data(project);
            }
        }
    }
    schema.execute(req).await.into()
}

async fn graphiql() -> impl IntoResponse {
    response::Html(http::GraphiQLSource::build().endpoint("/graphql").finish())
}

#[cfg(test)]
mod tests {
    #[test]
    fn schema() {}
}
