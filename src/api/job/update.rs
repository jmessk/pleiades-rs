use anyhow::{Context, Result};
use std::borrow::Cow;
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

pub struct JobUpdateBuilder<'a> {
    job_id: Option<Cow<'a, str>>,
    data_id: Option<Cow<'a, str>>,
    status: Option<Cow<'a, str>>,
}

impl<'a> JobUpdateBuilder<'a> {
    pub fn new() -> JobUpdateBuilder<'a> {
        JobUpdateBuilder {
            job_id: None,
            data_id: None,
            status: None,
        }
    }

    pub fn build(self) -> Result<JobUpdateRequest<'a>> {
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

    pub fn job_id(mut self, job_id: impl Into<Cow<'a, str>>) -> JobUpdateBuilder<'a> {
        self.job_id = Some(job_id.into());
        self
    }

    pub fn data_id(mut self, data_id: impl Into<Cow<'a, str>>) -> JobUpdateBuilder<'a> {
        self.data_id = Some(data_id.into());
        self
    }

    pub fn status(mut self, status: impl Into<Cow<'a, str>>) -> JobUpdateBuilder<'a> {
        self.status = Some(status.into());
        self
    }
}

#[derive(serde::Serialize, Debug)]
pub struct JobUpdateRequest<'a> {
    #[serde(skip_serializing)]
    job_id: Cow<'a, str>,
    #[serde(rename = "output")]
    data_id: Cow<'a, str>,
    #[serde(rename = "state")]
    job_status: Cow<'a, str>,
    status: Cow<'a, str>,
}

impl<'a> JobUpdateRequest<'a> {
    pub fn builder() -> JobUpdateBuilder<'a> {
        JobUpdateBuilder::new()
    }
}

impl<'a> MecrmRequest for JobUpdateRequest<'a> {
    type Response = JobUpdateResponse;

    async fn send(&self, client: &Arc<Client>) -> Result<JobUpdateResponse> {
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
            .with_context(|| "failed to update job status")
    }
}
