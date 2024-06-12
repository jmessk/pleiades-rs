use anyhow::{Context, Result};
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

pub struct LambdaCreateBuilder {
    client: Arc<Client>,
    data_id: Option<String>,
    runtime: Option<String>,
}

impl LambdaCreateBuilder {
    pub fn new(client: Arc<Client>) -> LambdaCreateBuilder {
        LambdaCreateBuilder {
            client,
            data_id: None,
            runtime: None,
        }
    }

    pub fn build(self) -> Result<LambdaCreateRequest> {
        let data_id = self.data_id.with_context(|| "data_id is required")?;
        let runtime = self.runtime.with_context(|| "runtime is required")?;

        Ok(LambdaCreateRequest {
            handler: self.client,
            data_id,
            runtime,
        })
    }

    pub fn data_id(mut self, data_id: impl Into<String>) -> LambdaCreateBuilder {
        self.data_id = Some(data_id.into());
        self
    }

    pub fn runtime(mut self, runtime: impl Into<String>) -> LambdaCreateBuilder {
        self.runtime = Some(runtime.into());
        self
    }
}

#[derive(serde::Serialize, Debug)]
pub struct LambdaCreateRequest {
    #[serde(skip_serializing)]
    handler: Arc<Client>,
    #[serde(rename = "codex")]
    data_id: String,
    runtime: String,
}

impl LambdaCreateRequest {
    pub fn builder(client: Arc<Client>) -> LambdaCreateBuilder {
        LambdaCreateBuilder::new(client)
    }
}

impl MecrmRequest for LambdaCreateRequest {
    type Response = LambdaCreateResponse;

    async fn send(&self) -> Result<LambdaCreateResponse> {
        let endpoint = "lambda";

        let response = self
            .handler
            .client()
            .post(self.handler.host().join(endpoint).unwrap())
            .json(&self)
            .send()
            .await?;

        LambdaCreateResponse::from_response(response).await
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct LambdaCreateResponse {
    pub code: u32,
    pub status: String,
    #[serde(rename = "id")]
    pub lambda_id: String,
}

impl MecrmResponse for LambdaCreateResponse {
    type Response = LambdaCreateResponse;

    async fn from_response(response: reqwest::Response) -> Result<LambdaCreateResponse> {
        response
            .json()
            .await
            .with_context(|| "Failed to parse response")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Client;

    #[tokio::test]
    async fn test_lambda_create() {
        let handler = Client::builder()
            .host("https://mecrm.dolylab.cc/api/v0.5-snapshot/")
            .build()
            .unwrap();

        let handler = Arc::new(handler);

        let request = LambdaCreateRequest::builder(handler.clone())
            .data_id("0")
            .runtime("test+mecrm-rs")
            .build()
            .unwrap();

        let response = request.send().await;
        assert!(response.is_ok());

        dbg!(response.unwrap());
    }
}
