use std::borrow::Cow;

use crate::api::{ApiError, ApiRequest, ApiResponse, Result};

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
pub struct Request<'a> {
    /// runtimes that the worker supports
    #[serde(rename = "runtime")]
    pub runtimes: &'a [&'a str],
}

impl<'a> ApiRequest for Request<'a> {
    type Response = Response;

    fn endpoint(&self) -> Cow<'static, str> {
        "worker".into()
    }

    async fn send(
        &self,
        client: &reqwest::Client,
        host: &url::Url,
    ) -> Result<Response> {
        let endpoint = host.join(&self.endpoint()).unwrap();
        let request = client.post(endpoint).json(self).build()?;

        tracing::debug!("registering worker: {:?}", self);

        let response = client.execute(request).await?;
        Response::from_response(response).await
    }
}

/// Response from registering a worker
#[derive(serde::Deserialize, Debug)]
pub struct Response {
    pub code: u32,
    pub status: String,

    /// worker ID
    #[serde(rename = "id")]
    pub worker_id: String,

    /// runtimes that the worker supports
    #[serde(rename = "runtime")]
    pub runtimes: Vec<String>,
}

impl ApiResponse for Response {
    type Response = Response;

    async fn from_response(response: reqwest::Response) -> Result<Response> {
        let body = response.text().await?;

        match serde_json::from_str::<Response>(&body) {
            Ok(response) => {
                tracing::debug!("worker registered: {}", body);
                Ok(response)
            }
            Err(_) => {
                tracing::error!("failed to register worker: {}", body);
                Err(ApiError::parse(&body))
            }
        }
    }
}
