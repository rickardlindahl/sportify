use axum::{routing::post, Router};
use dotenv::dotenv;

mod config;
mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let app_config = config::Config::new();

    let app = Router::new()
        .route("/api/teams", post(routes::teams::create_team))
        .with_state(app_config);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
