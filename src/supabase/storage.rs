use axum::{
    Router,
    extract::{Multipart, State},
    http::response,
    response::{IntoResponse, Response},
    routing::{post, put},
};
use garde::{Validate, rules::length::bytes};
use serde::{Deserialize, Serialize};

use crate::config::AppConfig;
use crate::valid::*;
use anyhow::Result;
use axum::extract::{Json, Path, Query};
use reqwest::{Client, StatusCode, header::HeaderMap};
#[derive(Deserialize, Serialize)]
pub struct OnPOSTSuccess {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Key")]
    pub key: String,
}

#[derive(Deserialize, Serialize)]
pub struct OnPOSTFailure {
    #[serde(rename = "statusCode")]
    pub statuscode: String,
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "message")]
    pub message: String,
}

pub async fn new_found_post(
    State(app_config): State<AppConfig>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut foundpost = FoundPost::default();

    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        match name.as_str() {
            "image_file_bytes" => {
                // ✅ Get file_name BEFORE consuming the field
                let file_name = field
                    .file_name()
                    .map(|f| f.to_string())
                    .unwrap_or_else(|| "untitled_image".to_string());

                // ✅ Now safely consume it
                let data = field.bytes().await.unwrap();
                println!("Length of `{}` is {} bytes", name, data.len());

                foundpost.postimage.image_file_name = file_name;
                foundpost.postimage.image_file_bytes = data;
            }
            "finder_name" => {
                let text = field.text().await.unwrap();
                println!("Length of `{}` is {} bytes", name, text.len());
                foundpost.finder_name = text;
            }
            "found_location" => {
                let text = field.text().await.unwrap();
                println!("Length of `{}` is {} bytes", name, text.len());
                foundpost.found_location = text;
            }
            "contact" => {
                let text = field.text().await.unwrap();
                println!("Length of `{}` is {} bytes", name, text.len());
                foundpost.contact = text;
            }
            _ => println!("Alien param found: {}", name),
        }
    }

    // ✅ Validate the final struct
    if let Err(e) = foundpost.validate() {
        eprintln!("Validation Error: {}", e);
        return (StatusCode::UNPROCESSABLE_ENTITY, "Data validation failure").into_response();
    }

    println!("Making a new post for {}", foundpost.finder_name);

    // ✅ Continue with your async handler (e.g., uploading to Supabase)
    make_new_post(&app_config,foundpost).await.into_response()
}

pub async fn make_new_post(
    app_config: &AppConfig,
    mut foundpost: FoundPost,
) -> (StatusCode, String) {
    // Load environment variables once
    let endpoint_url = &app_config.endpoint_url;
    let service_role = &app_config.service_role;
    let project_name = &app_config.project_name;
    let folder_name = "folder";
    // Create HTTP client once
    let client = Client::new();
    let mut results = Vec::new();
    let name = foundpost.postimage.image_file_name;
    // Construct upload URL
    let url = format!(
        "{}/object/testbucket/{}/{}",
        endpoint_url, folder_name, name
    );

    // Set headers
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "image/jpeg".parse().unwrap());
    headers.insert(
        "Authorization",
        format!("Bearer {}", service_role).parse().unwrap(),
    );
    let bytes = foundpost.postimage.image_file_bytes;

    // Send upload request
    let response = match client.post(&url).headers(headers).body(bytes).send().await {
        Ok(resp) => resp,
        Err(err) => {
            eprintln!("Upload failed for `{}`: {}", name, err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("❌ Upload failed for `{}`", name),
            );
        }
    };
    if response.status().is_success() {
        println!("✅ Uploaded `{}` successfully!", name);
        results.push(format!("✅ Uploaded `{}`", name));
        let x = response.json::<OnPOSTSuccess>().await.expect("");
        foundpost.postimage.image_file_url = format!("{}/object/public/{}", endpoint_url, x.key);
        println!("{}", foundpost.postimage.image_file_url);
    } else {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        eprintln!("❌ Upload failed for `{}`: {}", name, error_text);
        results.push(format!("❌ Upload failed for `{}`", name));
    }
    if results.is_empty() {
        (StatusCode::BAD_REQUEST, "No files uploaded".to_string())
    } else {
        (StatusCode::OK, results.join("\n"))
    }
}

pub async fn upload_multipart(
    State(app_config): State<AppConfig>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    // Load environment variables once
    let endpoint_url = &app_config.endpoint_url;
    let service_role = &app_config.service_role;
    let project_name = &app_config.project_name;
    let folder_name = "folder";
    // Create HTTP client once
    let client = Client::new();
    let mut results = Vec::new();

    // Process all fields
    while let Some(mut field) = multipart.next_field().await.unwrap_or(None) {
        let name = field.name().unwrap_or("unknown").to_string();

        let bytes = match field.bytes().await {
            Ok(b) => b,
            Err(e) => {
                eprintln!("Failed to read field `{}`: {}", name, e);
                results.push(format!("❌ Failed to read `{}`", name));
                continue;
            }
        };

        println!("Length of `{}` is {} bytes", name, bytes.len());

        // Construct upload URL
        let url = format!(
            "{}/object/testbucket/{}/{}",
            endpoint_url, folder_name, name
        );

        // Set headers
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "image/jpeg".parse().unwrap());
        headers.insert(
            "Authorization",
            format!("Bearer {}", service_role).parse().unwrap(),
        );

        // Send upload request
        let response = match client.post(&url).headers(headers).body(bytes).send().await {
            Ok(resp) => resp,
            Err(err) => {
                eprintln!("Upload failed for `{}`: {}", name, err);
                results.push(format!("❌ Upload failed for `{}`", name));
                continue;
            }
        };

        if response.status().is_success() {
            println!("✅ Uploaded `{}` successfully!", name);
            results.push(format!("✅ Uploaded `{}`", name));
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            eprintln!("❌ Upload failed for `{}`: {}", name, error_text);
            results.push(format!("❌ Upload failed for `{}`", name));
        }
    }

    if results.is_empty() {
        (StatusCode::BAD_REQUEST, "No files uploaded").into_response()
    } else {
        (StatusCode::OK, results.join("\n")).into_response()
    }
}

pub async fn upload_manual(State(app_config):State<AppConfig>) -> Result<()> {
    println!("umm");
    let endpoint_url = &app_config.endpoint_url;
    let service_role = &app_config.service_role;
    let project_name = &app_config.project_name;
    let file_path = "./image1.jpg";
    let client = Client::builder().build()?;
    let mut headers = HeaderMap::new();
    // headers.insert("apikey", service_role.parse()?);
    headers.insert("Content-Type", "image/jpeg".parse()?);
    headers.insert("Authorization", format!("Bearer {}", service_role).parse()?);
    let bytes = std::fs::read(file_path)?;
    let url = format!("{}/object/testbucket/folder/image1.jpg", endpoint_url);
    let request = client.post(&url).headers(headers).body(bytes);
    let response = request.send().await?;
    let status = response.status();
    let body = response.text().await?;
    println!("Status: {}", status);
    println!("Response: {}", body);
    Ok(())
}
