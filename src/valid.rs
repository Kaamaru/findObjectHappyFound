use axum::{body::Bytes, extract::{Query,State}, response::IntoResponse};
use axum_valid::Garde;
use garde::Validate;
use serde::Deserialize;
use crate::config::*;
#[derive(Debug)]
pub struct ValidatedBytes(Bytes);

#[derive(Debug, Validate)]
pub struct PostImage {
    #[garde(skip)]
    pub image_file_bytes: Bytes,
    #[garde(skip)]
    pub image_file_url: String,
    #[garde(skip)]
    pub image_file_name: String,
}
impl Default for PostImage {
    fn default() -> Self {
        Self {
            image_file_bytes: Bytes::default(),
            image_file_url: String::default(),
            image_file_name: String::default(),
        }
    }
}

#[derive(Debug, Validate)]
pub struct FoundPost {
    #[garde(length(min = 3))]
    pub finder_name: String,
    #[garde(skip)]
    pub postimage: PostImage,
    #[garde(length(min = 3))]
    pub found_location: String,
    #[garde(length(min = 3))]
    pub contact: String,
}

impl Default for FoundPost {
    fn default() -> Self {
        Self {
            finder_name: String::default(),
            postimage: PostImage::default(),
            found_location: String::default(),
            contact: String::default(),
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct SensitiveData {
    #[garde(range(min = 1, max = 10))]
    pub data_size: isize,
    #[garde(range(min = 2))]
    pub data_happiness: isize,
    #[garde(skip)]
    pub optional_field: Option<String>,
}

pub async fn form_query(
    State(app_config): State<AppConfig>,
    Garde(Query(sensitivedata)): Garde<Query<SensitiveData>>
) -> impl IntoResponse
{

    println!("{:?}", sensitivedata);
    "Ok."
}
