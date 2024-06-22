use anyhow::{Context, Result};
use std::borrow::Cow;
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

pub struct WorkerContractBuilder<'a> {
    worker_id: Option<Cow<'a, str>>,
    tags: Option<Vec<Cow<'a, str>>>,
    timeout: Option<u32>,
}

impl<'a> WorkerContractBuilder<'a> {
    pub fn new() -> WorkerContractBuilder<'a> {
        WorkerContractBuilder {
            worker_id: None,
            tags: None,
            timeout: None,
        }
    }

    pub fn build(self) -> Result<WorkerContractRequest<'a>> {
        let worker_id = self.worker_id.with_context(|| "worker_id is required")?;
        let tags = self.tags.unwrap_or_default();
        let timeout = self.timeout.unwrap_or(0);

        Ok(WorkerContractRequest {
            worker_id,
            tags,
            timeout,
        })
    }

    pub fn worker_id(mut self, worker_id: impl Into<Cow<'a, str>>) -> WorkerContractBuilder<'a> {
        self.worker_id = Some(worker_id.into());
        self
    }

    pub fn tags(mut self, tags: Vec<Cow<'a, str>>) -> WorkerContractBuilder<'a> {
        self.tags = Some(tags);
        self
    }

    pub fn timeout(mut self, timeout: u32) -> WorkerContractBuilder<'a> {
        self.timeout = Some(timeout);
        self
    }
}

#[derive(serde::Serialize, Debug)]
pub struct WorkerContractRequest<'a> {
    #[serde(rename = "id")]
    worker_id: Cow<'a, str>,
    tags: Vec<Cow<'a, str>>,
    timeout: u32,
}

impl<'a> WorkerContractRequest<'a> {
    pub fn builder() -> WorkerContractBuilder<'a> {
        WorkerContractBuilder::new()
    }
}

impl<'a> MecrmRequest for WorkerContractRequest<'a> {
    type Response = WorkerContractResponse;

    async fn send(&self, client: &Arc<Client>) -> Result<WorkerContractResponse> {
        let endpoint = format!("worker/{}/contract", self.worker_id);

        let response = client
            .client()
            .post(client.host().join(&endpoint).unwrap())
            .json(&self)
            .send()
            .await?;

        WorkerContractResponse::from_response(response).await
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct WorkerContractResponse {
    pub code: u32,
    pub status: String,
    #[serde(rename = "job")]
    pub job_id: Option<String>,
}

impl MecrmResponse for WorkerContractResponse {
    type Response = WorkerContractResponse;

    async fn from_response(response: reqwest::Response) -> Result<WorkerContractResponse> {
        response
            .json()
            .await
            .with_context(|| "failed to worker contract job")
    }
}
