use anyhow::{Context, Result};
use std::borrow::Cow;
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

#[derive(serde::Serialize, Debug, typed_builder::TypedBuilder)]
pub struct WorkerContractRequest<'a> {
    #[builder(setter(into))]
    #[serde(rename = "id")]
    worker_id: Cow<'a, str>,

    #[builder(default)]
    tags: Vec<Cow<'a, str>>,

    timeout: u32,
}

impl<'a> MecrmRequest for WorkerContractRequest<'a> {
    type Response = WorkerContractResponse;

    fn endpoint(&self, host: &url::Url) -> url::Url {
        host.join(&format!("worker/{}/contract", self.worker_id)).unwrap()
    }

    async fn send(&self, client: &Arc<Client>) -> Result<WorkerContractResponse> {
        let response = client
            .client()
            .post(self.endpoint(client.host()))
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
