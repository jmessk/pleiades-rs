use anyhow::Result;
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
        host.join(&format!("worker/{}/contract", self.worker_id))
            .unwrap()
    }

    async fn send(&self, client: &Arc<Client>) -> Result<WorkerContractResponse> {
        log::debug!("contracting worker: {:?}", self);

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
        let body = response.text().await?;

        match serde_json::from_str::<WorkerContractResponse>(&body) {
            Ok(response) => match &response.job_id {
                Some(job_id) => {
                    log::info!("worker contracted: {}", job_id);
                    log::debug!("worker contracted: {}", body);

                    Ok(response)
                }
                None => {
                    log::info!("worker contracted no job");
                    log::debug!("worker contracted no job: {}", body);

                    Ok(response)
                }
            },
            Err(e) => {
                log::error!("failed to contract worker: {}", body);
                anyhow::bail!("failed to contract worker: {}", e)
            }
        }
    }
}
