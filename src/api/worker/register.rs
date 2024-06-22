use anyhow::{Context, Result};
use std::borrow::Cow;
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

pub struct WorkerRegisterBuilder<'a> {
    runtimes: Option<Vec<Cow<'a, str>>>,
}

impl<'a> WorkerRegisterBuilder<'a> {
    pub fn new() -> WorkerRegisterBuilder<'a> {
        WorkerRegisterBuilder { runtimes: None }
    }

    pub fn build(self) -> Result<WorkerRegisterRequest<'a>> {
        let runtimes = self.runtimes.with_context(|| "runtimes is required")?;

        Ok(WorkerRegisterRequest { runtimes })
    }

    pub fn runtimes(mut self, runtimes: Vec<Cow<'a, str>>) -> WorkerRegisterBuilder<'a> {
        self.runtimes = Some(runtimes);
        self
    }
}

#[derive(serde::Serialize, Debug)]
pub struct WorkerRegisterRequest<'a> {
    #[serde(rename = "runtime")]
    runtimes: Vec<Cow<'a, str>>,
}

impl<'a> WorkerRegisterRequest<'a> {
    pub fn builder() -> WorkerRegisterBuilder<'a> {
        WorkerRegisterBuilder::new()
    }
}

impl<'a> MecrmRequest for WorkerRegisterRequest<'a> {
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
            .with_context(|| "failed to register worker")
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
            .runtimes(vec!["test1".into(), "test2".into()])
            .build()
            .unwrap();

        dbg!(&request);

        let response = request.send(&client).await;
        assert!(response.is_ok());

        dbg!(response.unwrap().runtimes);
    }
}
