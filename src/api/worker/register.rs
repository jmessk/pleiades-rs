use anyhow::{Context, Result};
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

pub struct WorkerRegisterBuilder {
    handler: Arc<Client>,
    runtimes: Option<Vec<String>>,
}

impl WorkerRegisterBuilder {
    pub fn new(handler: Arc<Client>) -> WorkerRegisterBuilder {
        WorkerRegisterBuilder {
            handler,
            runtimes: None,
        }
    }

    pub fn build(self) -> Result<WorkerRegisterRequest> {
        let runtimes = self.runtimes.with_context(|| "runtimes is required")?;

        Ok(WorkerRegisterRequest {
            handler: self.handler,
            runtimes,
        })
    }

    pub fn runtimes(mut self, runtimes: Vec<String>) -> WorkerRegisterBuilder {
        self.runtimes = Some(runtimes);
        self
    }
}

#[derive(serde::Serialize, Debug)]
pub struct WorkerRegisterRequest {
    #[serde(skip_serializing)]
    handler: Arc<Client>,
    #[serde(rename = "runtime")]
    runtimes: Vec<String>,
}

impl WorkerRegisterRequest {
    pub fn builder(handler: Arc<Client>) -> WorkerRegisterBuilder {
        WorkerRegisterBuilder::new(handler)
    }
}

impl MecrmRequest for WorkerRegisterRequest {
    type Response = WorkerRegisterResponse;

    async fn send(&self) -> Result<WorkerRegisterResponse> {
        let endpoint = "worker";

        let response = self
            .handler
            .client()
            .post(self.handler.host().join(endpoint).unwrap())
            .json(&self)
            .send()
            .await?;

        WorkerRegisterResponse::from_response(response).await
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct WorkerRegisterResponse {
    pub code: u32,
    pub status: String,
    #[serde(rename = "id")]
    pub worker_id: String,
    #[serde(rename = "runtime")]
    pub runtimes: Vec<String>,
}

impl MecrmResponse for WorkerRegisterResponse {
    type Response = WorkerRegisterResponse;

    async fn from_response(response: reqwest::Response) -> Result<WorkerRegisterResponse> {
        response
            .json()
            .await
            .with_context(|| "Failed to parse response")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Client;

    #[tokio::test]
    async fn test_worker_register() {
        let handler = Client::builder()
            .host("https://mecrm.dolylab.cc/api/v0.5-snapshot/")
            .build()
            .unwrap();

        let handler = Arc::new(handler);

        let request = WorkerRegisterRequest::builder(handler.clone())
            .runtimes(vec![
                "test1".to_string(),
                "test2".to_string(),
            ])
            .build()
            .unwrap();

        dbg!(&request);

        let response = request.send().await;
        assert!(response.is_ok());

        dbg!(response.unwrap().runtimes);
    }
}
