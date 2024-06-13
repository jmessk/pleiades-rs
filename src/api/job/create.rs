use anyhow::{Context, Result};
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

pub struct JobCreateBuilder {
    data_id: Option<String>,
    lambda_id: Option<String>,
    tags: Option<Vec<String>>,
}

impl JobCreateBuilder {
    pub fn new() -> JobCreateBuilder {
        JobCreateBuilder {
            data_id: None,
            lambda_id: None,
            tags: None,
        }
    }

    pub fn build(self) -> Result<JobCreateRequest> {
        let data_id = self.data_id.with_context(|| "data_id is required")?;
        let lambda_id = self.lambda_id.with_context(|| "lambda_id is required")?;
        let tags = self.tags.unwrap_or_default();

        Ok(JobCreateRequest {
            data_id,
            lambda_id,
            tags,
        })
    }

    pub fn data_id(mut self, data_id: impl Into<String>) -> JobCreateBuilder {
        self.data_id = Some(data_id.into());
        self
    }

    pub fn lambda_id(mut self, lambda_id: impl Into<String>) -> JobCreateBuilder {
        self.lambda_id = Some(lambda_id.into());
        self
    }

    pub fn tags(mut self, tags: Vec<String>) -> JobCreateBuilder {
        self.tags = Some(tags);
        self
    }
}

#[derive(serde::Serialize, Debug)]
pub struct JobCreateRequest {
    #[serde(rename = "input")]
    data_id: String,
    #[serde(rename = "lambda")]
    lambda_id: String,
    tags: Vec<String>,
}

impl JobCreateRequest {
    pub fn builder() -> JobCreateBuilder {
        JobCreateBuilder::new()
    }
}

impl MecrmRequest for JobCreateRequest {
    type Response = JobCreateResponse;

    async fn send(&self, client: &Arc<Client>) -> Result<JobCreateResponse> {
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

#[derive(serde::Deserialize, Debug)]
pub struct JobCreateResponse {
    pub code: i32,
    pub status: String,
    #[serde(rename = "id")]
    pub job_id: String,
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
