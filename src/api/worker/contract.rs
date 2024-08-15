use std::borrow::Cow;

use crate::api::{Error, ErrorResponse, Request, Response, Result};

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
    pub worker_id: Cow<'a, str>,

    /// tags to associate with contracting a job
    #[builder(default)]
    pub tags: Vec<Cow<'a, str>>,

    /// timeout in seconds to wait for a job
    pub timeout: u32,
}

impl<'a> Request for WorkerContractRequest<'a> {
    type Response = WorkerContractResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("worker/{}/contract", self.worker_id).into()
    }

    async fn send(
        &self,
        client: &reqwest::Client,
        host: &url::Url,
    ) -> Result<WorkerContractResponse> {
        let endpoint = host.join(&self.endpoint()).unwrap();
        let request = client.post(endpoint).json(&self).build()?;

        log::debug!("contracting worker: {:?}", self);

        let response = client.execute(request).await?;
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

impl Response for WorkerContractResponse {
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
            Err(_) => match serde_json::from_str::<ErrorResponse>(&body) {
                Ok(response) => {
                    log::error!("failed to contract job: {:?}", response);
                    Err(Error::Response(response))
                }
                Err(e) => Err(Error::Parse(e)),
            },
        }
    }
}
