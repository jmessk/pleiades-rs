use anyhow::{Context, Result};
use std::borrow::Cow;
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

#[derive(serde::Serialize, Debug, typed_builder::TypedBuilder)]
pub struct JobUpdateRequest<'a> {
    #[builder(setter(into))]
    #[serde(skip_serializing)]
    job_id: Cow<'a, str>,

    #[builder(setter(into))]
    #[serde(rename = "output")]
    data_id: Cow<'a, str>,

    #[builder(setter(into))]
    status: Cow<'a, str>,
}

impl<'a> MecrmRequest for JobUpdateRequest<'a> {
    type Response = JobUpdateResponse;

    fn endpoint(&self, host: &url::Url) -> url::Url {
        host.join(&format!("job/{}", self.job_id)).unwrap()
    }

    async fn send(&self, client: &Arc<Client>) -> Result<JobUpdateResponse> {
        let response = client
            .client()
            .post(self.endpoint(client.host()))
            .json(&self)
            .send()
            .await?;

        JobUpdateResponse::from_response(response).await
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct JobUpdateResponse {
    pub code: i32,
    pub status: String,
    pub message: String,
}

impl MecrmResponse for JobUpdateResponse {
    type Response = JobUpdateResponse;

    async fn from_response(response: reqwest::Response) -> Result<JobUpdateResponse> {
        response
            .json()
            .await
            .with_context(|| "failed to update job status")
    }
}
