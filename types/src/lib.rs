use std::collections::HashMap;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use users::User;
// Define `AppConfig` struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub client_id: String,
    pub client_secret: String,
    pub issuer: String,
    pub redirect_uris: Vec<String>,
}

// Define `Config` struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub apps: HashMap<String, AppConfig>,
    pub url: String,
    #[serde(skip_serializing)]
    pub load_error: Option<String>, // `error` in Go is a bit different, use `Option<String>` for optional errors
}

// Define `LoginRequest` struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub client_id: String,
    pub redirect_uri: String,
    pub scope: String,
    pub response_type: String,
    pub state: String,
    pub code_issued_at: SystemTime,  // `time.Time` in Go can be represented with `SystemTime`
    pub user: User,                // Assuming `User` struct exists in `pkg/users`
    pub app_config: AppConfig,
}

impl Config {
    pub fn new() -> Self {
        Config {
            apps: HashMap::new(),
            url: "http://localhost:8080".to_string(),
            load_error: None,
        }
    }
}