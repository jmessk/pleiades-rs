use anyhow::{Context, Result};
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Handler;

pub struct WorkerContractBuilder {
    handler: Arc<Handler>,
    worker_id: Option<String>,
    tags: Option<Vec<String>>,
    timeout: Option<u32>,
}

impl WorkerContractBuilder {
    pub fn new(handler: Arc<Handler>) -> WorkerContractBuilder {
        WorkerContractBuilder {
            handler,
            worker_id: None,
            tags: None,
            timeout: None,
        }
    }

    pub fn build(self) -> Result<WorkerContractRequest> {
        let worker_id = self.worker_id.with_context(|| "worker_id is required")?;
        let tags = self.tags.unwrap_or_default();
        let timeout = self.timeout.unwrap_or(0);

        Ok(WorkerContractRequest {
            handler: self.handler,
            worker_id,
            tags,
            timeout,
        })
    }

    pub fn worker_id(mut self, worker_id: impl Into<String>) -> WorkerContractBuilder {
        self.worker_id = Some(worker_id.into());
        self
    }

    pub fn tags(mut self, tags: Vec<String>) -> WorkerContractBuilder {
        self.tags = Some(tags);
        self
    }

    pub fn timeout(mut self, timeout: u32) -> WorkerContractBuilder {
        self.timeout = Some(timeout);
        self
    }
}

#[derive(serde::Serialize)]
pub struct WorkerContractRequest {
    #[serde(skip_serializing)]
    handler: Arc<Handler>,
    #[serde(rename = "id")]
    worker_id: String,
    tags: Vec<String>,
    timeout: u32,
}

impl WorkerContractRequest {
    pub fn builder(handler: Arc<Handler>) -> WorkerContractBuilder {
        WorkerContractBuilder::new(handler)
    }
}

impl MecrmRequest for WorkerContractRequest {
    type Response = WorkerContractResponse;

    async fn send(&self) -> Result<WorkerContractResponse> {
        let endpoint = format!("worker/{}/contract", self.worker_id);

        let response = self
            .handler
            .client()
            .post(self.handler.host().join(&endpoint).unwrap())
            .json(&self)
            .send()
            .await?;

        WorkerContractResponse::from_response(response).await
    }
}

#[derive(serde::Deserialize)]
pub struct WorkerContractResponse {
    pub code: u32,
    status: String,
    #[serde(rename = "id")]
    pub job_id: Option<String>,
}

impl MecrmResponse for WorkerContractResponse {
    type Response = WorkerContractResponse;

    async fn from_response(response: reqwest::Response) -> Result<WorkerContractResponse> {
        response
            .json()
            .await
            .with_context(|| "Failed to parse response")
    }
}
