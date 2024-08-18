use std::borrow::Cow;

use crate::api::{Error, ErrorResponse, Request, Response, Result};

///
#[derive(Debug, typed_builder::TypedBuilder)]
pub struct Get<'a> {
    ///
    #[builder(setter(into))]
    pub namespace_id: Cow<'a, str>,

    #[builder(setter(into))]
    pub key: Cow<'a, str>,
}

impl<'a> Request for Get<'a> {
    type Response = GetResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("kv/{}/field/{}", self.namespace_id, self.key).into()
    }

    async fn send(&self, client: &reqwest::Client, host: &url::Url) -> Result<GetResponse> {
        let endpoint = host.join(&self.endpoint()).unwrap();
        let request = client.get(endpoint).build()?;

        tracing::debug!("getting kv store: {:?}", self);

        let response = client.execute(request).await?;
        GetResponse::from_response(response).await
    }
}

/// Response from creating a lambda
#[derive(serde::Deserialize, Debug)]
pub struct GetResponse {
    pub code: u32,
    pub status: String,

    /// created lambda ID
    pub value: String,
}

impl Response for GetResponse {
    type Response = GetResponse;

    async fn from_response(response: reqwest::Response) -> Result<GetResponse> {
        let body = response.text().await?;

        match serde_json::from_str::<GetResponse>(&body) {
            Ok(response) => {
                tracing::debug!("got kv store: {}", body);
                Ok(response)
            }
            Err(_) => {
                tracing::error!("failed to get kv store: {}", body);

                match serde_json::from_str::<ErrorResponse>(&body) {
                    Ok(response) => Err(Error::Response(response)),
                    Err(e) => Err(Error::Parse(e)),
                }
            }
        }
    }
}
