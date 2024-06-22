use anyhow::{Context, Result};
use std::borrow::Cow;
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

pub struct LambdaCreateBuilder<'a> {
    data_id: Option<Cow<'a, str>>,
    runtime: Option<Cow<'a, str>>,
}

impl<'a> LambdaCreateBuilder<'a> {
    pub fn new() -> LambdaCreateBuilder<'a> {
        LambdaCreateBuilder {
            data_id: None,
            runtime: None,
        }
    }

    pub fn build(self) -> Result<LambdaCreateRequest<'a>> {
        let data_id = self.data_id.with_context(|| "data_id is required")?;
        let runtime = self.runtime.with_context(|| "runtime is required")?;

        Ok(LambdaCreateRequest { data_id, runtime })
    }

    pub fn data_id(mut self, data_id: impl Into<Cow<'a, str>>) -> LambdaCreateBuilder<'a> {
        self.data_id = Some(data_id.into());
        self
    }

    pub fn runtime(mut self, runtime: impl Into<Cow<'a, str>>) -> LambdaCreateBuilder<'a> {
        self.runtime = Some(runtime.into());
        self
    }
}

#[derive(serde::Serialize, Debug)]
pub struct LambdaCreateRequest<'a> {
    #[serde(rename = "codex")]
    data_id: Cow<'a, str>,
    runtime: Cow<'a, str>,
}

impl<'a> LambdaCreateRequest<'a> {
    pub fn builder() -> LambdaCreateBuilder<'a> {
        LambdaCreateBuilder::new()
    }
}

impl<'a> MecrmRequest for LambdaCreateRequest<'a> {
    type Response = LambdaCreateResponse;

    async fn send(&self, client: &Arc<Client>) -> Result<LambdaCreateResponse> {
        let endpoint = "lambda";

        let response = client
            .client()
            .post(client.host().join(endpoint).unwrap())
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
        let client = Client::builder()
            .host("https://mecrm.dolylab.cc/api/v0.5-snapshot/")
            .build()
            .unwrap();

        let client = Arc::new(client);

        let request = LambdaCreateRequest::builder()
            .data_id("0")
            .runtime("test+mecrm-rs")
            .build()
            .unwrap();

        let response = request.send(&client).await;
        assert!(response.is_ok());

        dbg!(response.unwrap());
    }
}
