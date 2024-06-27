use anyhow::Result;
use std::borrow::Cow;

use crate::api::{MecrmRequest, MecrmResponse};

/// Request to create a lambda
///
/// # Example
///
/// ```rust
/// use mecrs::api::lambda::create::LambdaCreateRequest;
///
/// let request = LambdaCreateRequest::builder()
///     .data_id("1")
///     .runtime("mecrs+test")
///     .build();
///
/// let response = request.send(&client).await?;
/// let lambda_id = response.lambda_id;
/// ```
#[derive(serde::Serialize, Debug, typed_builder::TypedBuilder)]
pub struct LambdaCreateRequest<'a> {
    /// blob data id as lambda code
    #[builder(setter(into))]
    #[serde(rename = "codex")]
    data_id: Cow<'a, str>,

    /// runtime that supports the lambda
    #[builder(setter(into))]
    runtime: Cow<'a, str>,
}

impl<'a> MecrmRequest for LambdaCreateRequest<'a> {
    type Response = LambdaCreateResponse;

    fn endpoint(&self) -> String {
        "lambda".to_string()
    }

    async fn send(
        &self,
        client: &reqwest::Client,
        host: &url::Url,
    ) -> Result<LambdaCreateResponse> {
        log::debug!("creating lambda: {:?}", self);

        let endpoint = host.join(&self.endpoint()).unwrap();
        let response = client.post(endpoint).json(&self).send().await?;

        LambdaCreateResponse::from_response(response).await
    }
}

/// Response from creating a lambda
#[derive(serde::Deserialize, Debug)]
pub struct LambdaCreateResponse {
    pub code: u32,
    pub status: String,

    /// created lambda ID
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
