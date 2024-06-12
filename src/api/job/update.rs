use anyhow::{Context, Result};
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Handler;

pub struct JobUpdateBuilder {
    handler: Arc<Handler>,
    job_id: Option<String>,
    data_id: Option<String>,
    status: Option<String>,
    job_status: Option<String>,
}

impl JobUpdateBuilder {
    pub fn new(handler: Arc<Handler>) -> JobUpdateBuilder {
        JobUpdateBuilder {
            handler,
            job_id: None,
            data_id: None,
            status: None,
            job_status: None,
        }
    }

    pub fn build(self) -> Result<JobUpdateRequest> {
        let job_id = self.job_id.with_context(|| "job_id is required")?;
        let data_id = self.data_id.with_context(|| "data_id is required")?;
        let status = self.status.with_context(|| "status is required")?;
        let job_status = self.job_status.with_context(|| "job_status is required")?;

        Ok(JobUpdateRequest {
            handler: self.handler,
            job_id,
            data_id,
            status,
            job_status,
        })
    }

    pub fn job_id(mut self, job_id: impl Into<String>) -> JobUpdateBuilder {
        self.job_id = Some(job_id.into());
        self
    }

    pub fn data_id(mut self, data_id: impl Into<String>) -> JobUpdateBuilder {
        self.data_id = Some(data_id.into());
        self
    }

    pub fn status(mut self, status: impl Into<String>) -> JobUpdateBuilder {
        self.status = Some(status.into());
        self
    }

    pub fn job_status(mut self, job_status: impl Into<String>) -> JobUpdateBuilder {
        self.job_status = Some(job_status.into());
        self
    }
}

#[derive(serde::Serialize)]
pub struct JobUpdateRequest {
    #[serde(skip_serializing)]
    handler: Arc<Handler>,
    #[serde(skip_serializing)]
    job_id: String,
    #[serde(rename = "output")]
    data_id: String,
    status: String,
    #[serde(rename = "state")]
    job_status: String,
}

impl JobUpdateRequest {
    pub fn builder(handler: Arc<Handler>) -> JobUpdateBuilder {
        JobUpdateBuilder::new(handler)
    }
}

impl MecrmRequest for JobUpdateRequest {
    type Response = JobUpdateResponse;

    async fn send(&self) -> Result<JobUpdateResponse> {
        let endpoint = format!("job/{}", self.job_id);

        let response = self
            .handler
            .client()
            .post(self.handler.host().join(&endpoint).unwrap())
            .json(&self)
            .send()
            .await?;

        JobUpdateResponse::from_response(response).await
    }
}

#[derive(serde::Deserialize)]
pub struct JobUpdateResponse {
    code: i32,
    status: String,
    message: String,
}

impl MecrmResponse for JobUpdateResponse {
    type Response = JobUpdateResponse;

    async fn from_response(response: reqwest::Response) -> Result<JobUpdateResponse> {
        response
            .json()
            .await
            .with_context(|| "Failed to parse response")
    }
}
