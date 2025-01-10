use std::borrow::Cow;

use crate::api::{ApiError, ApiRequest, ApiResponse, Result};

#[derive(Debug, typed_builder::TypedBuilder)]
pub struct Request<'a> {
    #[builder(setter(into))]
    pub namespace_id: Cow<'a, str>,

    #[builder(setter(into))]
    pub key: Cow<'a, str>,
}

impl<'a> ApiRequest for Request<'a> {
    type Response = Response;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("kv/{}/field/{}", self.namespace_id, self.key).into()
    }

    async fn send(&self, client: &reqwest::Client, host: &url::Url) -> Result<Response> {
        let endpoint = host.join(&self.endpoint()).unwrap();
        let request = client.get(endpoint).build()?;

        tracing::debug!("getting kv store: {:?}", self);

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
    pub value: String,
}

impl ApiResponse for Response {
    type Response = Response;

    async fn from_response(response: reqwest::Response) -> Result<Response> {
        let body = response.text().await?;

        match serde_json::from_str::<Response>(&body) {
            Ok(response) => {
                tracing::debug!("got kv store: {}", body);
                Ok(response)
            }
            Err(_) => {
                tracing::error!("failed to get kv store: {}", body);
                Err(ApiError::parse(&body))
            }
        }
    }
}
