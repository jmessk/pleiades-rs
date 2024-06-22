use anyhow::{Context, Result};
use std::borrow::Cow;
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

pub struct JobCreateBuilder<'a> {
    data_id: Option<Cow<'a, str>>,
    lambda_id: Option<Cow<'a, str>>,
    tags: Option<Vec<Cow<'a, str>>>,
}

impl<'a> JobCreateBuilder<'a> {
    pub fn new() -> JobCreateBuilder<'a> {
        JobCreateBuilder {
            data_id: None,
            lambda_id: None,
            tags: None,
        }
    }

    pub fn build(self) -> Result<JobCreateRequest<'a>> {
        let data_id = self.data_id.with_context(|| "data_id is required")?;
        let lambda_id = self.lambda_id.with_context(|| "lambda_id is required")?;
        let tags = self.tags.unwrap_or_default();

        Ok(JobCreateRequest {
            data_id,
            lambda_id,
            tags,
        })
    }

    pub fn data_id(mut self, data_id: impl Into<Cow<'a, str>>) -> JobCreateBuilder<'a> {
        self.data_id = Some(data_id.into());
        self
    }

    pub fn lambda_id(mut self, lambda_id: impl Into<Cow<'a, str>>) -> JobCreateBuilder<'a> {
        self.lambda_id = Some(lambda_id.into());
        self
    }

    pub fn tags(mut self, tags: Vec<Cow<'a, str>>) -> JobCreateBuilder<'a> {
        self.tags = Some(tags);
        self
    }
}

#[derive(serde::Serialize, Debug)]
pub struct JobCreateRequest<'a> {
    #[serde(rename = "input")]
    data_id: Cow<'a, str>,
    #[serde(rename = "lambda")]
    lambda_id: Cow<'a, str>,
    tags: Vec<Cow<'a, str>>,
}

impl<'a> JobCreateRequest<'a> {
    pub fn builder() -> JobCreateBuilder<'a> {
        JobCreateBuilder::new()
    }
}

impl<'a> MecrmRequest for JobCreateRequest<'a> {
    type Response = JobCreateResponse;

    async fn send(&self, client: &Arc<Client>) -> Result<JobCreateResponse> {
        let endpoint = "job";

        let response = client
            .client()
            .post(client.host().join(endpoint).unwrap())
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
        response
            .json()
            .await
            .with_context(|| "failed to create job")
    }
}
