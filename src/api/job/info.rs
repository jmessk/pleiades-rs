use anyhow::{Context, Result};
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Handler;

struct JobInfoBuilder {
    handler: Arc<Handler>,
    job_id: Option<String>,
    except: Option<String>,
    timeout: Option<u32>,
}

impl JobInfoBuilder {
    pub fn new(handler: Arc<Handler>) -> JobInfoBuilder {
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
}

pub struct JobInfoRequest {
    handler: Arc<Handler>,
    job_id: String,
    except: Option<String>,
    timeout: Option<u32>,
}

impl JobInfoRequest {
    pub fn builder(handler: Arc<Handler>) -> JobInfoBuilder {
        JobInfoBuilder::new(handler)
    }
}

#[derive(serde::Deserialize)]
struct Lambda {
    #[serde(rename = "id")]
    lambda_id: String,
    runtime: String,
    #[serde(rename = "codex")]
    data_id: String,
}

#[derive(serde::Deserialize)]
struct Input {
    #[serde(rename = "id")]
    data_id: String,
}

#[derive(serde::Deserialize)]
struct Output {
    #[serde(rename = "id")]
    data_id: String,
}

#[derive(serde::Deserialize)]
pub struct JobInfoResponse {
    code: i32,
    status: String,
    #[serde(rename = "id")]
    job_id: String,
}

impl MecrmRequest for JobInfoRequest {
    type Response = JobInfoResponse;

    async fn send(&self) -> Result<JobInfoResponse> {
        let endpoint = format!("job/{}", self.job_id);

        let response = self
            .handler
            .client()
            .get(self.handler.host().join(&endpoint).unwrap())
            .send()
            .await?;

        JobInfoResponse::from_response(response).await
    }
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
