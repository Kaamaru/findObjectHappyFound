#![allow(warnings)]


use axum::{
    Router,
    extract::Multipart,
    http::response,
    response::{Html,IntoResponse, Response},
    routing::{get, post, put},
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
    let url = format!("0.0.0.0:{}",config::PORT);
    let listener = tokio::net::TcpListener::bind(url).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
fn get_router() -> Router {
    dotenv().ok();

    let app_config = config::AppConfig::from_env();

    let app = Router::new()
        .route("/upload", post(upload_multipart))
        .route("/hello", put(hello))
        .route("/form", get(form_query))
        .route("/ping", get(ping))
        .route("/html", get(render_html))
        .route("/newpost", post(new_found_post))
        .with_state(app_config);
    return app;
}


pub async fn render_html()-> impl IntoResponse{
    println!("pinged from client!!!!!!!!");
    Html(include_str!("../index.html"))
}
pub async fn ping()-> impl IntoResponse{
    println!("pinged from client!!!!!!!!");
    "pong!"
}
use garde::{rules::required::Required, Validate};
#[derive(Serialize, Deserialize,Validate)]
struct TestPayload {
    #[garde(required)]
    happy: Option<bool>,
}


async fn hello(Json(payload): Json<TestPayload>) -> impl IntoResponse {
    if let Some(value) = payload.happy {
        println!("{}", value);
        return value.to_string();
    } else {
        eprintln!("Error: `happy` field is missing!");
        return "Error: `happy` field is missing!".to_string();
    }

}
