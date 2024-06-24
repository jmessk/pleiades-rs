use anyhow::Result;
use std::borrow::Cow;
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

#[derive(serde::Serialize, Debug, typed_builder::TypedBuilder)]
pub struct JobCreateRequest<'a> {
    #[builder(setter(into))]
    #[serde(rename = "input")]
    data_id: Cow<'a, str>,

    #[builder(setter(into))]
    #[serde(rename = "lambda")]
    lambda_id: Cow<'a, str>,

    #[builder(default)]
    tags: Vec<Cow<'a, str>>,
}

impl<'a> MecrmRequest for JobCreateRequest<'a> {
    type Response = JobCreateResponse;

    fn endpoint(&self, host: &url::Url) -> url::Url {
        host.join("job").unwrap()
    }

    async fn send(&self, client: &Arc<Client>) -> Result<JobCreateResponse> {
        log::debug!("creating job: {:?}", self);

        let response = client
            .client()
            .post(self.endpoint(client.host()))
            .json(&self)
            .send()
            .await?;

        JobCreateResponse::from_response(response).await
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct JobCreateResponse {
    pub code: i32,
    pub status: String,
    #[serde(rename = "id")]
    pub job_id: String,
}

impl MecrmResponse for JobCreateResponse {
    type Response = JobCreateResponse;

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
