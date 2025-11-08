#![allow(warnings)]

use axum::{
    Router,
    extract::Multipart,
    http::response,
    response::{IntoResponse, Response},
    routing::{post, put,get},
};
use serde::{Deserialize, Serialize};

use axum::extract::{Json, Path, Query};
use dotenv::dotenv;
use reqwest::{Client, StatusCode, header::HeaderMap};
use std::env;

pub mod supabase;
use supabase::{database::*, storage::*};
pub mod valid;
use valid::*;
pub mod systems;
pub use systems::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // upload_manual().await;
    let app = get_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

fn get_router() -> Router {
    dotenv().ok();

    let app_config = config::AppConfig::from_env();
    println!("{:?}", app_config);

    let app = Router::new()
        .route("/upload", post(upload_multipart))
        .route("/hello", put(hello))
        .route("/form", get(form_query))
        .route("/newpost", post(new_found_post))
        .with_state(app_config);
    return app;
}

#[derive(Serialize, Deserialize)]
struct Payload {
    happy: bool,
}

async fn hello(Json(payload): Json<Payload>) -> impl IntoResponse {
    println!("{}", payload.happy);
    "Hello World"
}
