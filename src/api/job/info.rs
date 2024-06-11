use anyhow::{Context, Result};
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

pub struct JobInfoRequest {
    job_id: String,
}

#[derive(serde::Deserialize)]
struct Lambda {
    #[serde(rename = "id")]
    lambda_id: String,
    runtime: String,
    #[serde(rename = "codex")]
    data_id: String,
}

#[derive(serde::Deserialize)]
struct Input {
    #[serde(rename = "id")]
    data_id: String,
}

#[derive(serde::Deserialize)]
struct Output {
    #[serde(rename = "id")]
    data_id: String,
}

#[derive(serde::Deserialize)]
pub struct JobInfoResponse {
    code: i32,
    status: String,
    #[serde(rename = "id")]
    job_id: String,
}

impl JobInfoRequest {
    pub fn new(job_id: impl Into<String>) -> JobInfoRequest {
        JobInfoRequest {
            job_id: job_id.into(),
        }
    }
}

impl MecrmRequest for JobInfoRequest {
    type Response = JobInfoResponse;

    async fn send(self, client: Arc<Client>) -> Result<JobInfoResponse> {
        let endpoint = format!("job/{}", self.job_id);

        let response = client
            .client()
            .get(client.host().join(&endpoint).unwrap())
            .send()
            .await?;

        JobInfoResponse::from_response(response).await
    }
}

impl MecrmResponse for JobInfoResponse {
    type Response = JobInfoResponse;

    async fn from_response(response: reqwest::Response) -> Result<JobInfoResponse> {
        response
            .json()
            .await
            .with_context(|| "Failed to parse response")
    }
}
