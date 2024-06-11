use anyhow::{Context, Result};
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

#[derive(serde::Serialize)]
pub struct JobCreateRequest {
    data_id: String,
    lambda_id: String,
    tags: Vec<String>,
}

#[derive(serde::Deserialize)]
pub struct JobCreateResponse {
    code: i32,
    status: String,
    #[serde(rename = "id")]
    job_id: String,
}

impl JobCreateRequest {
    pub fn new(data_id: impl Into<String>, lambda_id: impl Into<String>, tags: Vec<String>) -> JobCreateRequest {
        JobCreateRequest {
            data_id: data_id.into(),
            lambda_id: lambda_id.into(),
            tags,
        }
    }
}

impl MecrmRequest for JobCreateRequest {
    type Response = JobCreateResponse;

    async fn send(self, client: Arc<Client>) -> Result<JobCreateResponse> {
        let endpoint = "job";

        let response = client
            .client()
            .post(client.host().join(endpoint).unwrap())
            .json(&self)
            .send()
            .await?;

        JobCreateResponse::from_response(response).await
    }
}

impl MecrmResponse for JobCreateResponse {
    type Response = JobCreateResponse;

    async fn from_response(response: reqwest::Response) -> Result<JobCreateResponse> {
        response
            .json()
            .await
            .with_context(|| "Failed to parse response")
    }
}