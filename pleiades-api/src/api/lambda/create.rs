use std::borrow::Cow;

use crate::api::{Error, error, CoreRequest, CoreResponse, Result};

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
pub struct Request<'a> {
    /// blob data id as lambda code
    #[builder(setter(into))]
    #[serde(rename = "codex")]
    pub data_id: Cow<'a, str>,

    /// runtime that supports the lambda
    #[builder(setter(into))]
    pub runtime: Cow<'a, str>,
}

impl<'a> CoreRequest for Request<'a> {
    type Response = Response;

    fn endpoint(&self) -> Cow<'static, str> {
        "lambda".into()
    }

    async fn send(
        &self,
        client: &reqwest::Client,
        host: &url::Url,
    ) -> Result<Response> {
        let endpoint = host.join(&self.endpoint()).unwrap();
        let request = client.post(endpoint).json(self).build()?;

        tracing::debug!("creating lambda: {:?}", self);

        let response = client.execute(request).await?;
        Response::from_response(response).await
    }
}

/// Response from creating a lambda
#[derive(serde::Deserialize, Debug)]
pub struct Response {
    pub code: u32,
    pub status: String,

    /// created lambda ID
    #[serde(rename = "id")]
    pub lambda_id: String,
}

impl CoreResponse for Response {
    type Response = Response;

    async fn from_response(response: reqwest::Response) -> Result<Response> {
        let body = response.text().await?;

        match serde_json::from_str::<Response>(&body) {
            Ok(response) => {
                tracing::debug!("lambda created: {}", body);
                Ok(response)
            }
            Err(_) => {
                tracing::error!("failed to create lambda: {}", body);

                match serde_json::from_str::<error::Response>(&body) {
                    Ok(response) => Err(Error::Response(response)),
                    Err(e) => Err(Error::Parse(e)),
                }
            }
        }
    }
}
