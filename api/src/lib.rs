use std::env;

use axum::{Json, Router};
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::get;

use service::Fetcher;

use crate::response::TokenResponse;

mod response;

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    //TODO
    let app = Router::new()
        .route("/:address", get(find_token));

    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}

async fn find_token(Path(address): Path<String>) -> impl IntoResponse {
    let metadata = Fetcher::fetch_token_metadata(address.as_str()).await.unwrap();
    let top_holders = Fetcher::fetch_top_holders(address.as_str(), 10).await.unwrap();
    let info = Fetcher::fetch_token_info(address.as_str()).await.unwrap();
    let extensions = Fetcher::fetch_token_extension_info(address.as_str()).await.unwrap();

    let response = TokenResponse {
        mint: address,
        metadata,
        extensions,
        info,
        top_holders,
    };

    Json(response)
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}