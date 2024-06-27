use anyhow::Result;
use std::borrow::Cow;

use crate::api::{MecrmRequest, MecrmResponse};

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
    data_id: Cow<'a, str>,

    /// lambda ID to execute the job
    #[builder(setter(into))]
    #[serde(rename = "lambda")]
    lambda_id: Cow<'a, str>,

    /// tags to associate with the job
    ///
    /// default: empty
    #[builder(default)]
    tags: Vec<Cow<'a, str>>,
}

impl<'a> MecrmRequest for JobCreateRequest<'a> {
    type Response = JobCreateResponse;

    fn endpoint(&self) -> String {
        "job".to_string()
    }

    async fn send(&self, client: &reqwest::Client, host: &url::Url) -> Result<JobCreateResponse> {
        log::debug!("creating job: {:?}", self);

        let endpoint = host.join(&self.endpoint()).unwrap();
        let response = client.post(endpoint).json(&self).send().await?;

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

impl MecrmResponse for JobCreateResponse {
    type Response = JobCreateResponse;

    // #[tracing::instrument]
    async fn from_response(response: reqwest::Response) -> Result<JobCreateResponse> {
        let body = response.text().await?;

        match serde_json::from_str::<JobCreateResponse>(&body) {
            Ok(response) => {
                log::info!("job created");
                log::debug!("job created: {}", body);

                Ok(response)
            }
            Err(e) => {
                log::error!("failed to create job: {}", body);
                anyhow::bail!("failed to create job: {}", e)
            }
        }
    }
}
