use anyhow::{Context, Result};
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

pub struct JobInfoBuilder {
    handler: Arc<Client>,
    job_id: Option<String>,
    except: Option<String>,
    timeout: Option<u32>,
}

impl JobInfoBuilder {
    pub fn new(handler: Arc<Client>) -> JobInfoBuilder {
        JobInfoBuilder {
            handler,
            job_id: None,
            except: None,
            timeout: None,
        }
    }

    pub fn build(self) -> Result<JobInfoRequest> {
        let job_id = self.job_id.with_context(|| "job_id is required")?;

        Ok(JobInfoRequest {
            handler: self.handler,
            job_id,
            except: self.except,
            timeout: self.timeout,
        })
    }

    pub fn job_id(mut self, job_id: impl Into<String>) -> JobInfoBuilder {
        self.job_id = Some(job_id.into());
        self
    }

    pub fn except(mut self, except: impl Into<String>) -> JobInfoBuilder {
        self.except = Some(except.into());
        self
    }

    pub fn timeout(mut self, timeout: u32) -> JobInfoBuilder {
        self.timeout = Some(timeout);
        self
    }
}

#[derive(Debug)]
pub struct JobInfoRequest {
    handler: Arc<Client>,
    job_id: String,
    except: Option<String>,
    timeout: Option<u32>,
}

impl JobInfoRequest {
    pub fn builder(handler: Arc<Client>) -> JobInfoBuilder {
        JobInfoBuilder::new(handler)
    }
}

impl MecrmRequest for JobInfoRequest {
    type Response = JobInfoResponse;

    async fn send(&self) -> Result<JobInfoResponse> {
        let endpoint = format!("job/{}", self.job_id);

        let response = self
            .handler
            .client()
            .get(self.handler.host().join(&endpoint).unwrap())
            .query(&[("except", self.except.as_deref())])
            .query(&[("timeout", self.timeout)])
            .send()
            .await?;

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
