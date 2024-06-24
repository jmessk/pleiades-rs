use anyhow::Result;
use std::borrow::Cow;
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

#[derive(serde::Serialize, Debug, typed_builder::TypedBuilder)]
pub struct LambdaCreateRequest<'a> {
    #[builder(setter(into))]
    #[serde(rename = "codex")]
    data_id: Cow<'a, str>,

    #[builder(setter(into))]
    runtime: Cow<'a, str>,
}

impl<'a> MecrmRequest for LambdaCreateRequest<'a> {
    type Response = LambdaCreateResponse;

    fn endpoint(&self, host: &url::Url) -> url::Url {
        host.join("lambda").unwrap()
    }

    async fn send(&self, client: &Arc<Client>) -> Result<LambdaCreateResponse> {
        log::debug!("creating lambda: {:?}", self);

        let response = client
            .client()
            .post(self.endpoint(client.host()))
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
        let body = response.text().await?;

        match serde_json::from_str::<LambdaCreateResponse>(&body) {
            Ok(response) => {
                log::info!("lambda created");
                log::debug!("lambda created: {}", body);

                Ok(response)
            }
            Err(e) => {
                log::error!("failed to create lambda: {}", body);
                anyhow::bail!("failed to create lambda: {}", e)
            }
        }
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
            .build();

        let client = Arc::new(client);

        let request = LambdaCreateRequest::builder()
            .data_id("0")
            .runtime("test+mecrm-rs")
            .build();

        let response = request.send(&client).await;
        assert!(response.is_ok());

        dbg!(response.unwrap());
    }
}
