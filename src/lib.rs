mod request;
mod error;
pub mod axum;

use std::env;
use prost::Message;
use reqwest::{header::CONTENT_TYPE, Client};

pub use request::ViewUserRequest;
pub use request::ViewProjectRequest;
pub use error::Error;

pub struct MetricsClient {
    http_client: Client,
    base_url: String,
}

impl MetricsClient {
    pub fn new(http_client: Client, base_url: String) -> Self {
        Self {
            http_client,
            base_url,
        }
    }

    pub async fn view_user(&self, view_user_request: ViewUserRequest) -> Result<(), Error> {
        self.http_client
            .post(format!("{}/metrics/users/view", self.base_url))
            .header(CONTENT_TYPE, "application/octet-stream")
            .body(view_user_request.encode_to_vec())
            .send()
            .await?
            .error_for_status()
            .map(|_| ())
            .map_err(Error::from)
    }

    pub async fn view_project(&self, view_project_request: ViewProjectRequest) -> Result<(), Error> {
        self.http_client
            .post(format!("{}/metrics/projects/view", self.base_url))
            .header(CONTENT_TYPE, "application/octet-stream")
            .body(view_project_request.encode_to_vec())
            .send()
            .await?
            .error_for_status()
            .map(|_| ())
            .map_err(Error::from)
    }
}

impl Default for MetricsClient {
    fn default() -> Self {
        let base_url = env::var("METRICS_BASE_URL")
            .unwrap_or("http://metrics".to_string())
            .trim_end_matches('/')
            .to_string();

        Self { 
            http_client: Default::default(),
            base_url
        }
    }
}