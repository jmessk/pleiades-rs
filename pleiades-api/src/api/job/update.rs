use std::borrow::Cow;

use crate::api::{ApiError, ApiRequest, ApiResponse, Result};

/// Request to update a job
///
/// # Example
///
/// ```rust
/// use mecrs::api::job::update::JobUpdateRequest;
///
/// let request = JobUpdateRequest::builder()
///    .job_id("1")
///    .data_id("2")
///    .status("Finished")
///    .build();
///
/// let response = request.send(&client).await?;
/// ```
#[derive(serde::Serialize, Debug, typed_builder::TypedBuilder)]
pub struct Request<'a> {
    /// job ID to update
    #[builder(setter(into))]
    #[serde(skip_serializing)]
    pub job_id: Cow<'a, str>,

    /// output data ID
    #[builder(setter(into))]
    #[serde(rename = "output")]
    pub data_id: Cow<'a, str>,

    /// job status to set
    #[builder(setter(into))]
    pub status: Cow<'a, str>,
}

impl<'a> ApiRequest for Request<'a> {
    type Response = Response;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("job/{}", self.job_id).into()
    }

    async fn send(&self, client: &reqwest::Client, host: &url::Url) -> Result<Response> {
        let endpoint = host.join(&self.endpoint()).unwrap();
        let request = client.post(endpoint).json(self).build()?;

        tracing::debug!("updating job: {:?}", self);

        let response = client.execute(request).await?;
        Response::from_response(response).await
    }
}

/// Response from updating a job
#[derive(serde::Deserialize, Debug)]
pub struct Response {
    pub code: i32,
    pub status: String,
    pub message: String,
}

impl ApiResponse for Response {
    type Response = Response;

    async fn from_response(response: reqwest::Response) -> Result<Response> {
        let body = response.text().await?;

        match serde_json::from_str::<Response>(&body) {
            Ok(response) => {
                tracing::debug!("job status updated: {}", body);
                Ok(response)
            }
            Err(_) => {
                tracing::error!("failed to update job status: {}", body);
                Err(ApiError::parse(&body))
            }
        }
    }
}
