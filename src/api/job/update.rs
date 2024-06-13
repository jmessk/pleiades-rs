use anyhow::{Context, Result};
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

pub struct JobUpdateBuilder {
    job_id: Option<String>,
    data_id: Option<String>,
    status: Option<String>,
}

impl JobUpdateBuilder {
    pub fn new() -> JobUpdateBuilder {
        JobUpdateBuilder {
            job_id: None,
            data_id: None,
            status: None,
        }
    }

    pub fn build(self) -> Result<JobUpdateRequest> {
        let job_id = self.job_id.with_context(|| "job_id is required")?;
        let data_id = self.data_id.with_context(|| "data_id is required")?;
        let status = self.status.with_context(|| "status is required")?;

        Ok(JobUpdateRequest {
            job_id,
            data_id,
            job_status: status.clone(),
            status,
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
}

#[derive(serde::Serialize, Debug)]
pub struct JobUpdateRequest {
    #[serde(skip_serializing)]
    job_id: String,
    #[serde(rename = "output")]
    data_id: String,
    #[serde(rename = "state")]
    job_status: String,
    status: String,
}

impl JobUpdateRequest {
    pub fn builder() -> JobUpdateBuilder {
        JobUpdateBuilder::new()
    }
}

impl MecrmRequest for JobUpdateRequest {
    type Response = JobUpdateResponse;

    async fn send(&self, client: Arc<Client>) -> Result<JobUpdateResponse> {
        let endpoint = format!("job/{}", self.job_id);

        let response = client
            .client()
            .post(client.host().join(&endpoint).unwrap())
            .json(&self)
            .send()
            .await?;

        JobUpdateResponse::from_response(response).await
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct JobUpdateResponse {
    pub code: i32,
    pub status: String,
    pub message: String,
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
