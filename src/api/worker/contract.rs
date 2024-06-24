use anyhow::Result;
use std::borrow::Cow;
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

/// Request to contract a worker
/// 
/// # Example
/// 
/// ```rust
/// use mecrs::api::worker::contract::WorkerContractRequest;
/// 
/// let request = WorkerContractRequest::builder()
///     .worker_id("1")
///     .timeout(10)
///     .build();
/// 
/// let response = request.send(&client).await?;
/// let job_id = match response.job_id {
///     Some(job_id) => job_id,
///     None => {
///        println!("no job contracted");
///        return;
///    }
/// 
/// // worker can be contracted with tags
/// let request_with_tags = WorkerContractRequest::builder()
///     .worker_id("1")
///     .tags(vec!["gpu".into(), "fpga".into()])
///     .timeout(10)
///     .build();
/// ```
#[derive(serde::Serialize, Debug, typed_builder::TypedBuilder)]
pub struct WorkerContractRequest<'a> {
    /// worker ID using to contract
    #[builder(setter(into))]
    #[serde(rename = "id")]
    worker_id: Cow<'a, str>,

    /// tags to associate with contracting a job
    #[builder(default)]
    tags: Vec<Cow<'a, str>>,

    /// timeout in seconds to wait for a job
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

/// Response from contracting a worker
#[derive(serde::Deserialize, Debug)]
pub struct WorkerContractResponse {
    pub code: u32,
    pub status: String,

    /// contracted job ID. if no job is contracted, this field is `None`
    #[serde(rename = "job")]
    pub job_id: Option<String>,
}

impl MecrmResponse for WorkerContractResponse {
    type Response = WorkerContractResponse;

    async fn from_response(response: reqwest::Response) -> Result<WorkerContractResponse> {
        let body = response.text().await?;

        match serde_json::from_str::<WorkerContractResponse>(&body) {
            Ok(response) => match &response.job_id {
                Some(_) => {
                    log::info!("job contracted");
                    log::debug!("job contracted: {}", body);

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
