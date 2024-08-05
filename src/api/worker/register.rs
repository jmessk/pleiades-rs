use std::borrow::Cow;

use crate::api::{Error, ErrorResponse, Request, Response, Result};

/// Request to register a worker
///
/// # Example
///
/// ```rust
/// use mecrs::api::worker::register::WorkerRegisterRequest;
///
/// let request = WorkerRegisterRequest::builder()
///     .runtimes(vec!["test1".into(), "test2".into()])
///     .build();
///
/// let response = request.send(&client).await?;
/// ```
#[derive(serde::Serialize, Debug, typed_builder::TypedBuilder)]
pub struct WorkerRegisterRequest<'a> {
    /// runtimes that the worker supports
    #[serde(rename = "runtime")]
    runtimes: Vec<Cow<'a, str>>,
}

impl<'a> Request for WorkerRegisterRequest<'a> {
    type Response = WorkerRegisterResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        "worker".into()
    }

    async fn send(
        self,
        client: &reqwest::Client,
        host: &url::Url,
    ) -> Result<WorkerRegisterResponse> {
        let endpoint = host.join(&self.endpoint()).unwrap();
        let request = client.post(endpoint).json(&self).build()?;

        log::debug!("registering worker: {:?}", self);

        let response = client.execute(request).await?;
        WorkerRegisterResponse::from_response(response).await
    }
}

/// Response from registering a worker
#[derive(serde::Deserialize, Debug)]
pub struct WorkerRegisterResponse {
    pub code: u32,
    pub status: String,

    /// worker ID
    #[serde(rename = "id")]
    pub worker_id: String,

    /// runtimes that the worker supports
    #[serde(rename = "runtime")]
    pub runtimes: Vec<String>,
}

impl Response for WorkerRegisterResponse {
    type Response = WorkerRegisterResponse;

    async fn from_response(response: reqwest::Response) -> Result<WorkerRegisterResponse> {
        let body = response.text().await?;

        match serde_json::from_str::<WorkerRegisterResponse>(&body) {
            Ok(response) => {
                log::info!("worker registered");
                log::debug!("worker registered: {}", body);

                Ok(response)
            }
            Err(_) => match serde_json::from_str::<ErrorResponse>(&body) {
                Ok(response) => {
                    log::error!("failed to register worker: {:?}", response);
                    Err(Error::Response(response))
                }
                Err(e) => Err(Error::Parse(e)),
            },
        }
    }
}
