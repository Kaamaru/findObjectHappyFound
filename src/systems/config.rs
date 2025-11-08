use std::env;

pub const PORT: i16 = 3000;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub endpoint_url: String,
    pub service_role: String,
    pub project_name: String,
    pub bucket_name: String,
    pub happynot_database_password: String,
    pub database1_name: String,
    pub database_port: String,
}
impl AppConfig {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        Self {
            endpoint_url: env::var("ENDPOINT_URL").expect("ENDPOINT_URL must be set"),
            service_role: env::var("SERVICE_ROLE").expect("SERVICE_ROLE must be set"),
            project_name: env::var("PROJECT_NAME").unwrap_or_else(|_| "default".to_string()),
            bucket_name: env::var("BUCKET_NAME").unwrap_or_else(|_| "default".to_string()),
            happynot_database_password: env::var("HAPPYNOT_DATABASE_PASSWORD")
                .unwrap_or_else(|_| "default".to_string()),
            database1_name: env::var("DATABASE1_NAME").unwrap_or_else(|_| "default".to_string()),
            database_port: env::var("DATABASE_PORT").unwrap_or_else(|_| "default".to_string()),
        }
    }
}
use axum::extract::FromRef;

impl FromRef<AppConfig> for () {
    fn from_ref(_: &AppConfig) -> Self {
        ()
    }
}
