use anyhow::Result;
use std::borrow::Cow;
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

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
pub struct JobUpdateRequest<'a> {
    /// job ID to update
    #[builder(setter(into))]
    #[serde(skip_serializing)]
    job_id: Cow<'a, str>,

    /// output data ID
    #[builder(setter(into))]
    #[serde(rename = "output")]
    data_id: Cow<'a, str>,

    /// job status to set
    #[builder(setter(into))]
    status: Cow<'a, str>,
}

impl<'a> MecrmRequest for JobUpdateRequest<'a> {
    type Response = JobUpdateResponse;

    fn endpoint(&self, host: &url::Url) -> url::Url {
        host.join(&format!("job/{}", self.job_id)).unwrap()
    }

    async fn send(&self, client: &Arc<Client>) -> Result<JobUpdateResponse> {
        log::debug!("updating job: {:?}", self);

        let response = client
            .client()
            .post(self.endpoint(client.host()))
            .json(&self)
            .send()
            .await?;

        JobUpdateResponse::from_response(response).await
    }
}

/// Response from updating a job
#[derive(serde::Deserialize, Debug)]
pub struct JobUpdateResponse {
    pub code: i32,
    pub status: String,
    pub message: String,
}

impl MecrmResponse for JobUpdateResponse {
    type Response = JobUpdateResponse;

    async fn from_response(response: reqwest::Response) -> Result<JobUpdateResponse> {
        let body = response.text().await?;

        match serde_json::from_str::<JobUpdateResponse>(&body) {
            Ok(response) => {
                log::info!("job updated");
                log::debug!("job updated: {}", body);

                Ok(response)
            }
            Err(e) => {
                log::error!("failed to update job: {}", body);
                anyhow::bail!("failed to update job: {}", e);
            }
        }
    }
}
