use crate::apis::configuration::Configuration;
use crate::apis::default_api;
use crate::models::AuthAuthTokenPostRequest;
use std::sync::Mutex;

const SANDBOX_URL: &str = "https://sandbox-api.praemunio.org/api/v1";
const LIVE_URL: &str = "https://api.praemunio.org/api/v1";

/// High-level Praemunio client with authentication and token management.
pub struct PraemunioClient {
    pub_key: String,
    secret_key: String,
    base_url: String,
    access_token: Mutex<Option<String>>,
    refresh_token: Mutex<Option<String>>,
}

impl PraemunioClient {
    /// Create a new client pointing to the sandbox environment.
    pub fn sandbox(public_key: &str, secret_key: &str) -> Self {
        Self::new(public_key, secret_key, SANDBOX_URL)
    }

    /// Create a new client pointing to the live environment.
    pub fn live(public_key: &str, secret_key: &str) -> Self {
        Self::new(public_key, secret_key, LIVE_URL)
    }

    fn new(public_key: &str, secret_key: &str, base_url: &str) -> Self {
        Self {
            pub_key: public_key.to_string(),
            secret_key: secret_key.to_string(),
            base_url: base_url.trim_end_matches('/').to_string(),
            access_token: Mutex::new(None),
            refresh_token: Mutex::new(None),
        }
    }

    /// Authenticate with public/secret keys and store the access/refresh tokens.
    pub fn authenticate(&self) -> Result<(), String> {
        let config = Configuration {
            base_path: self.base_url.clone(),
            ..Default::default()
        };
        let req = AuthAuthTokenPostRequest {
            public_key: self.pub_key.clone(),
            secret_key: self.secret_key.clone(),
        };

        rt_block_on(async {
            let response = default_api::auth_auth_token_post(&config, req)
                .await
                .map_err(|e| e.to_string())?;

            if let Some(result) = response.get("result").and_then(|v| v.as_object()) {
                let access = result
                    .get("access_token")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let refresh = result
                    .get("refresh_token")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                *self.access_token.lock().unwrap() = Some(access);
                *self.refresh_token.lock().unwrap() = Some(refresh);
                Ok(())
            } else {
                Err("Authentication failed: no result in response".to_string())
            }
        })
    }

    /// Get the current access token.
    pub fn get_access_token(&self) -> Option<String> {
        self.access_token.lock().unwrap().clone()
    }

    /// Get the current refresh token.
    pub fn get_refresh_token(&self) -> Option<String> {
        self.refresh_token.lock().unwrap().clone()
    }
}

/// Minimal tokio runtime for blocking auth in sync context.
fn rt_block_on<F: std::future::Future>(future: F) -> F::Output {
    if let Ok(handle) = tokio::runtime::Handle::try_current() {
        handle.block_on(future)
    } else {
        let rt = tokio::runtime::Runtime::new().expect("failed to create tokio runtime");
        rt.block_on(future)
    }
}
