use anyhow::{Context, Result};
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

#[derive(serde::Serialize)]
pub struct JobUpdateRequest {
    #[serde(skip_serializing)]
    job_id: String,
    data_id: String,
    status: String,
    job_status: String,
}

#[derive(serde::Deserialize)]
pub struct JobUpdateResponse {
    code: i32,
    status: String,
    message: String,
}

impl JobUpdateRequest {
    pub fn new(
        job_id: impl Into<String>,
        data_id: impl Into<String>,
        status: impl Into<String>,
        job_status: impl Into<String>,
    ) -> JobUpdateRequest {
        JobUpdateRequest {
            job_id: job_id.into(),
            data_id: data_id.into(),
            status: status.into(),
            job_status: job_status.into(),
        }
    }
}

impl MecrmRequest for JobUpdateRequest {
    type Response = JobUpdateResponse;

    async fn send(self, client: Arc<Client>) -> Result<JobUpdateResponse> {
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

impl MecrmResponse for JobUpdateResponse {
    type Response = JobUpdateResponse;

    async fn from_response(response: reqwest::Response) -> Result<JobUpdateResponse> {
        response
            .json()
            .await
            .with_context(|| "Failed to parse response")
    }
}
