use axum::{Json, extract::State, response::IntoResponse};
use chrono::{DateTime, Utc};
use garde::{Validate, rules::length::bytes};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::{FromRow, Pool, Postgres};

use crate::*;
use anyhow::Result;
use config::AppConfig;
use dotenv::dotenv;
use reqwest::{Client, StatusCode, header::HeaderMap};

use std::env;

/////////
#[derive(Debug, Deserialize)]

pub struct CreatePost {
    general_description: Option<String>,
    finder_name: Option<String>,
    found_location: Option<String>,
    contact: Option<String>,
    image_file_link: Option<String>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct Post {
    id: i64,
    created_at: DateTime<Utc>,
    general_description: String,
    finder_name: String,
    found_location: String,
    contact: String,
    image_file_link: Option<String>,
}

///////////

pub async fn post_database(
    State(app_config): State<AppConfig>,
                           Json(new_post): Json<CreatePost>,
) -> Result<Json<Post>, StatusCode> {
    let database_password = &app_config.happynot_database_password;
    let project_name = &app_config.project_name;
    let encoded_password = urlencoding::encode(database_password);
    let database_url_full = format!(
        "postgresql://postgres:{}@db.{}.supabase.co:{}/postgres",
        encoded_password, project_name, 5432
    );
    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url_full)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let post = create_post(&pool, new_post)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(post))
}

async fn create_post(pool: &Pool<Postgres>, new_post: CreatePost) -> Result<Post, sqlx::Error> {
    let post = sqlx::query_as::<_, Post>(
        "INSERT INTO happynot (general_description, finder_name, found_location, contact, image_file_link)
    VALUES ($1, $2, $3, $4, $5)
    RETURNING id, created_at, general_description, finder_name, found_location, contact, image_file_link"
    )
    .bind(&new_post.general_description)
    .bind(&new_post.finder_name)
    .bind(&new_post.found_location)
    .bind(&new_post.contact)
    .bind(&new_post.image_file_link)
    .fetch_one(pool)
    .await?;

    Ok(post)
}

pub async fn get_database(State(app_config): State<AppConfig>) -> Result<Json<Post>, String> {
    let database1_name = &app_config.database1_name;
    let happynot_database_password = &app_config.happynot_database_password;
    let project_name = &app_config.project_name;
    let database_port = &app_config.database_port;

    let database_url_full = format!(
        "postgresql://postgres:{}@db.{}.supabase.co:{}/postgres",
        happynot_database_password, project_name, database_port
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url_full)
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    let post_id = 1;

    let post = get_something(&pool, &database1_name, post_id)
        .await
        .map_err(|e| format!("Failed to fetch post: {}", e))?;

    Ok(Json(post))
}

async fn get_something(
    pool: &Pool<Postgres>,
    database_name: &String,
    id: i32,
) -> Result<Post, sqlx::Error> {
    if !database_name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_')
    {
        return Err(sqlx::Error::Protocol("Invalid table name".into()));
    }

    let sql_query = format!(
        "SELECT id, created_at, general_description, finder_name,
        found_location, contact, image_file_link
        FROM {}
        WHERE id = $1",
        database_name
    );

    let post = sqlx::query_as::<_, Post>(&sql_query)
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(post)
}

// async fn get_all_posts(
//     pool: &Pool<Postgres>,
//     database_name: &String,
// ) -> Result<Vec<Post>, sqlx::Error> {
//     let sql_query = format!(
//         "SELECT id, created_at, general_description, finder_name,
//                             found_location, contact, image_file_link
//                             FROM {} ORDER BY created_at DESC",
//         database_name
//     );
//     let posts = sqlx::query_as::<_, Post>(&sql_query)
//         .fetch_all(pool)
//         .await?;
//
//     Ok(posts)
// }
