use anyhow::{Context, Result};
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

#[derive(serde::Serialize)]
pub struct LambdaCreateRequest {
    #[serde(rename = "codex")]
    data_id: String,
    runtime: String,
}

#[derive(serde::Deserialize)]
pub struct LambdaCreateResponse {
    pub code: u32,
    pub status: String,
    #[serde(rename = "id")]
    pub lambda_id: String,
}

impl LambdaCreateRequest {
    pub fn new(data_id: impl Into<String>, runtime: impl Into<String>) -> LambdaCreateRequest {
        LambdaCreateRequest {
            data_id: data_id.into(),
            runtime: runtime.into(),
        }
    }
}

impl MecrmRequest for LambdaCreateRequest {
    type Response = LambdaCreateResponse;

    async fn send(self, client: Arc<Client>) -> Result<LambdaCreateResponse> {
        let endpoint = "lambda";

        let response = client
            .client()
            .post(client.host().join(endpoint).unwrap())
            .json(&self)
            .send()
            .await?;

        LambdaCreateResponse::from_response(response).await
    }
}

impl MecrmResponse for LambdaCreateResponse {
    type Response = LambdaCreateResponse;

    async fn from_response(response: reqwest::Response) -> Result<LambdaCreateResponse> {
        response
            .json()
            .await
            .with_context(|| "Failed to parse response")
    }
}
