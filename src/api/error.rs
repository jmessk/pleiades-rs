use anyhow::{Context, Result};

use crate::api::MecrmResponse;

#[derive(serde::Deserialize)]
pub struct ErrorResponse {
    code: i32,
    status: String,
    message: String,
}

impl MecrmResponse for ErrorResponse {
    async fn from_response(response: reqwest::Response) -> Result<ErrorResponse> {
        response.json().await.context("Failed to parse response")
    }
}
