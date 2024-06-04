use anyhow::{Context, Result};

use crate::api::MecrmResponse;

#[derive(serde::Deserialize)]
pub struct MecrmErrorResponse {
    code: i32,
    status: String,
    message: String,
}

impl MecrmResponse for MecrmErrorResponse {
    async fn from_response(response: reqwest::Response) -> Result<MecrmErrorResponse> {
        response.json().await.context("Failed to parse response")
    }
}
