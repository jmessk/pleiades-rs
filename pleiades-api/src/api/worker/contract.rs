use std::borrow::Cow;

use crate::api::{Error, CoreRequest, CoreResponse, Result};

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
pub struct Request<'a> {
    /// worker ID using to contract
    #[builder(setter(into))]
    #[serde(rename = "id")]
    pub worker_id: Cow<'a, str>,

    /// tags to associate with contracting a job
    #[builder(default)]
    pub tags: &'a [&'a str],

    /// timeout in seconds to wait for a job
    pub timeout: u64,
}

impl<'a> CoreRequest for Request<'a> {
    type Response = Response;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("worker/{}/contract", self.worker_id).into()
    }

    async fn send(
        &self,
        client: &reqwest::Client,
        host: &url::Url,
    ) -> Result<Response> {
        let endpoint = host.join(&self.endpoint()).unwrap();
        let request = client.post(endpoint).json(self).build()?;

        tracing::debug!("contracting worker: {:?}", self);

        let response = client.execute(request).await?;
        Response::from_response(response).await
    }
}

/// Response from contracting a worker
#[derive(serde::Deserialize, Debug)]
pub struct Response {
    pub code: u32,
    pub status: String,

    /// contracted job ID. if no job is contracted, this field is `None`
    #[serde(rename = "job")]
    pub job_id: Option<String>,
}

impl CoreResponse for Response {
    type Response = Response;

    async fn from_response(response: reqwest::Response) -> Result<Response> {
        let body = response.text().await?;

        match serde_json::from_str::<Response>(&body) {
            Ok(response) => match &response.job_id {
                Some(_) => {
                    tracing::debug!("job contracted: {}", body);
                    Ok(response)
                }
                None => {
                    tracing::debug!("worker contracted no job: {}", body);
                    Ok(response)
                }
            },
            Err(_) => {
                tracing::error!("failed to contract job: {}", body);
                Err(Error::parse(&body))
            }
        }
    }
}
