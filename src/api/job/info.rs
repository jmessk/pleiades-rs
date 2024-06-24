use anyhow::Result;
use std::borrow::Cow;
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

#[derive(Debug, typed_builder::TypedBuilder)]
pub struct JobInfoRequest<'a> {
    #[builder(setter(into))]
    job_id: Cow<'a, str>,

    #[builder(default, setter(strip_option, into))]
    except: Option<Cow<'a, str>>,

    #[builder(default, setter(strip_option))]
    timeout: Option<u32>,
}

impl<'a> MecrmRequest for JobInfoRequest<'a> {
    type Response = JobInfoResponse;

    fn endpoint(&self, host: &url::Url) -> url::Url {
        host.join(&format!("job/{}", self.job_id)).unwrap()
    }

    async fn send(&self, client: &Arc<Client>) -> Result<JobInfoResponse> {
        log::debug!("getting job info: {:?}", self);

        let response = match &self.except {
            Some(except) => {
                client
                    .client()
                    .get(self.endpoint(client.host()))
                    .query(&[("except", except)])
                    .query(&[("timeout", self.timeout)])
                    .send()
                    .await?
            }
            None => {
                client
                    .client()
                    .get(self.endpoint(client.host()))
                    .send()
                    .await?
            }
        };

        JobInfoResponse::from_response(response).await
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct Lambda {
    #[serde(rename = "id")]
    pub lambda_id: String,
    pub runtime: String,
    #[serde(rename = "codex")]
    pub data_id: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Input {
    #[serde(rename = "id")]
    pub data_id: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Output {
    #[serde(rename = "id")]
    pub data_id: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct JobInfoResponse {
    pub code: i32,
    pub status: String,
    #[serde(rename = "id")]
    pub job_id: String,
    #[serde(rename = "state")]
    pub job_status: String,
    pub lambda: Lambda,
    pub input: Input,
    pub output: Option<Output>,
}

impl MecrmResponse for JobInfoResponse {
    type Response = JobInfoResponse;

    async fn from_response(response: reqwest::Response) -> Result<JobInfoResponse> {
        let body = response.text().await?;

        match serde_json::from_str::<JobInfoResponse>(&body) {
            Ok(response) => {
                log::info!("got job info");
                log::debug!("job info: {}", body);

                Ok(response)
            }
            Err(e) => {
                log::error!("failed to get job info: {}", body);
                anyhow::bail!("failed to get job info: {}", e)
            }
        }
    }
}
