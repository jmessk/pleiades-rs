use std::borrow::Cow;

use crate::api::{Error, ErrorResponse, Request, Response, Result};

/// Request to create a job
///
/// # Example
///
/// ```rust
/// use mecrs::api::job::create::JobCreateRequest;
///
/// let request = JobCreateRequest::builder()
///    .data_id("1")
///    .lambda_id("2")
///    .build();
///
/// let response = request.send(&client).await?;
///
/// let request_with_tags = JobCreateRequest::builder()
///     .data_id("1")
///     .lambda_id("2")
///     .tags(vec!["gpu".into(), "fpga".into()])
///     .build();
/// ```
#[derive(serde::Serialize, Debug, typed_builder::TypedBuilder)]
pub struct JobCreateRequest<'a> {
    /// blob data ID as job input
    #[builder(setter(into))]
    #[serde(rename = "input")]
    pub data_id: Cow<'a, str>,

    /// lambda ID to execute the job
    #[builder(setter(into))]
    #[serde(rename = "lambda")]
    pub lambda_id: Cow<'a, str>,

    /// tags to associate with the job
    ///
    /// default: empty
    #[builder(default)]
    pub tags: &'a [&'a str],
}

impl<'a> Request for JobCreateRequest<'a> {
    type Response = JobCreateResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        "job".into()
    }

    async fn send(&self, client: &reqwest::Client, host: &url::Url) -> Result<JobCreateResponse> {
        let endpoint = host.join(&self.endpoint()).unwrap();
        let request = client.post(endpoint).json(self).build()?;

        tracing::debug!("creating job: {:?}", self);

        let response = client.execute(request).await?;
        JobCreateResponse::from_response(response).await
    }
}

/// Response from creating a job
#[derive(serde::Deserialize, Debug)]
pub struct JobCreateResponse {
    pub code: i32,
    pub status: String,

    /// created job ID
    #[serde(rename = "id")]
    pub job_id: String,
}

impl Response for JobCreateResponse {
    type Response = JobCreateResponse;

    async fn from_response(response: reqwest::Response) -> Result<JobCreateResponse> {
        let body = response.text().await?;

        match serde_json::from_str::<JobCreateResponse>(&body) {
            Ok(response) => {
                tracing::debug!("job created: {:?}", response);
                Ok(response)
            }
            Err(_) => {
                tracing::error!("failed to create job: {}", body);

                match serde_json::from_str::<ErrorResponse>(&body) {
                    Ok(response) => Err(Error::Response(response)),
                    Err(e) => Err(Error::Parse(e)),
                }
            }
        }
    }
}
