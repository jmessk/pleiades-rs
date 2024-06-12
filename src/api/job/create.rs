use anyhow::{Context, Result};
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Handler;

pub struct JobCreateBuilder {
    handler: Arc<Handler>,
    data_id: Option<String>,
    lambda_id: Option<String>,
    tags: Option<Vec<String>>,
}

impl JobCreateBuilder {
    pub fn new(handler: Arc<Handler>) -> JobCreateBuilder {
        JobCreateBuilder {
            handler,
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
            handler: self.handler,
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

#[derive(serde::Serialize)]
pub struct JobCreateRequest {
    #[serde(skip_serializing)]
    handler: Arc<Handler>,
    #[serde(rename = "input")]
    data_id: String,
    #[serde(rename = "lambda")]
    lambda_id: String,
    tags: Vec<String>,
}

impl JobCreateRequest {
    pub fn builder(handler: Arc<Handler>) -> JobCreateBuilder {
        JobCreateBuilder::new(handler)
    }
}

impl MecrmRequest for JobCreateRequest {
    type Response = JobCreateResponse;

    async fn send(&self) -> Result<JobCreateResponse> {
        let endpoint = "job";

        let response = self
            .handler
            .client()
            .post(self.handler.host().join(endpoint).unwrap())
            .json(&self)
            .send()
            .await?;

        JobCreateResponse::from_response(response).await
    }
}

#[derive(serde::Deserialize)]
pub struct JobCreateResponse {
    code: i32,
    status: String,
    #[serde(rename = "id")]
    job_id: String,
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
