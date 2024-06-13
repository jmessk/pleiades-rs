use anyhow::{Context, Result};
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

pub struct WorkerRegisterBuilder {
    runtimes: Option<Vec<String>>,
}

impl WorkerRegisterBuilder {
    pub fn new() -> WorkerRegisterBuilder {
        WorkerRegisterBuilder { runtimes: None }
    }

    pub fn build(self) -> Result<WorkerRegisterRequest> {
        let runtimes = self.runtimes.with_context(|| "runtimes is required")?;

        Ok(WorkerRegisterRequest { runtimes })
    }

    pub fn runtimes(mut self, runtimes: Vec<String>) -> WorkerRegisterBuilder {
        self.runtimes = Some(runtimes);
        self
    }
}

#[derive(serde::Serialize, Debug)]
pub struct WorkerRegisterRequest {
    #[serde(rename = "runtime")]
    runtimes: Vec<String>,
}

impl WorkerRegisterRequest {
    pub fn builder() -> WorkerRegisterBuilder {
        WorkerRegisterBuilder::new()
    }
}

impl MecrmRequest for WorkerRegisterRequest {
    type Response = WorkerRegisterResponse;

    async fn send(&self, client: &Arc<Client>) -> Result<WorkerRegisterResponse> {
        let endpoint = "worker";

        let response = client
            .client()
            .post(client.host().join(endpoint).unwrap())
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
        let client = Client::builder()
            .host("https://mecrm.dolylab.cc/api/v0.5-snapshot/")
            .build()
            .unwrap();

        let client = Arc::new(client);

        let request = WorkerRegisterRequest::builder()
            .runtimes(vec!["test1".to_string(), "test2".to_string()])
            .build()
            .unwrap();

        dbg!(&request);

        let response = request.send(&client).await;
        assert!(response.is_ok());

        dbg!(response.unwrap().runtimes);
    }
}
