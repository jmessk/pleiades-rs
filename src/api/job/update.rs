use std::borrow::Cow;

use crate::api::{Error, ErrorResponse, Request, Response, Result};

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

impl<'a> Request for JobUpdateRequest<'a> {
    type Response = JobUpdateResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("job/{}", self.job_id).into()
    }

    async fn send(self, client: &reqwest::Client, host: &url::Url) -> Result<JobUpdateResponse> {
        let endpoint = host.join(&self.endpoint()).unwrap();
        let request = client.post(endpoint).json(&self).build()?;

        log::debug!("updating job: {:?}", self);

        let response = client.execute(request).await?;
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

impl Response for JobUpdateResponse {
    type Response = JobUpdateResponse;

    async fn from_response(response: reqwest::Response) -> Result<JobUpdateResponse> {
        let body = response.text().await?;

        match serde_json::from_str::<JobUpdateResponse>(&body) {
            Ok(response) => {
                log::info!("job status updated");
                log::debug!("job status updated: {}", body);

                Ok(response)
            }
            Err(_) => match serde_json::from_str::<ErrorResponse>(&body) {
                Ok(response) => {
                    log::error!("failed to update job status: {:?}", response);
                    Err(Error::Response(response))
                }
                Err(e) => Err(Error::Parse(e)),
            },
        }
    }
}
