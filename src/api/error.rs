use anyhow::{Context, Result};

use crate::api::MecrmResponse;

#[derive(serde::Deserialize)]
pub struct ErrorResponse {
    pub code: i32,
    pub status: String,
    pub message: String,
}

impl MecrmResponse for ErrorResponse {
    type Response = ErrorResponse;

    async fn from_response(response: reqwest::Response) -> Result<ErrorResponse> {
        response.json().await.context("Failed to parse response")
    }
}
