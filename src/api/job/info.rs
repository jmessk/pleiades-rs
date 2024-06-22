use anyhow::{Context, Result};
use std::borrow::Cow;
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

pub struct JobInfoBuilder<'a> {
    job_id: Option<Cow<'a, str>>,
    except: Option<Cow<'a, str>>,
    timeout: Option<u32>,
}

impl<'a> JobInfoBuilder<'a> {
    pub fn new() -> JobInfoBuilder<'a> {
        JobInfoBuilder {
            job_id: None,
            except: None,
            timeout: None,
        }
    }

    pub fn build(self) -> Result<JobInfoRequest<'a>> {
        let job_id = self.job_id.with_context(|| "job_id is required")?;

        Ok(JobInfoRequest {
            job_id,
            except: self.except,
            timeout: self.timeout,
        })
    }

    pub fn job_id(mut self, job_id: impl Into<Cow<'a, str>>) -> JobInfoBuilder<'a> {
        self.job_id = Some(job_id.into());
        self
    }

    pub fn except(mut self, except: impl Into<Cow<'a, str>>) -> JobInfoBuilder<'a> {
        self.except = Some(except.into());
        self
    }

    pub fn timeout(mut self, timeout: u32) -> JobInfoBuilder<'a> {
        self.timeout = Some(timeout);
        self
    }
}

#[derive(Debug)]
pub struct JobInfoRequest<'a> {
    job_id: Cow<'a, str>,
    except: Option<Cow<'a, str>>,
    timeout: Option<u32>,
}

impl<'a> JobInfoRequest<'a> {
    pub fn builder() -> JobInfoBuilder<'a> {
        JobInfoBuilder::new()
    }
}

impl<'a> MecrmRequest for JobInfoRequest<'a> {
    type Response = JobInfoResponse;

    async fn send(&self, client: &Arc<Client>) -> Result<JobInfoResponse> {
        let endpoint = format!("job/{}", self.job_id);

        let response = match &self.except {
            Some(except) => {
                client
                    .client()
                    .get(client.host().join(&endpoint).unwrap())
                    .query(&[("except", except)])
                    .query(&[("timeout", self.timeout)])
                    .send()
                    .await?
            }
            None => {
                client
                    .client()
                    .get(client.host().join(&endpoint).unwrap())
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
        response
            .json()
            .await
            .with_context(|| "Failed to parse response")
    }
}
