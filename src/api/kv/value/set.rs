use std::borrow::Cow;

use crate::api::{Error, ErrorResponse, Request, Response, Result};

///
#[derive(serde::Serialize, Debug, typed_builder::TypedBuilder)]
pub struct Set<'a> {
    ///
    #[builder(setter(into))]
    #[serde(skip)]
    pub namespace_id: Cow<'a, str>,

    #[builder(setter(into))]
    #[serde(skip)]
    pub key: Cow<'a, str>,

    #[builder(setter(into))]
    pub value: Cow<'a, str>,

    #[builder(default)]
    pub expire: u32,

    #[builder(setter(strip_bool))]
    pub get: bool,

    #[builder(setter(strip_bool))]
    pub append: bool,
}

impl<'a> Request for Set<'a> {
    type Response = GetResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("kv/{}/field/{}", self.namespace_id, self.key).into()
    }

    async fn send(&self, client: &reqwest::Client, host: &url::Url) -> Result<GetResponse> {
        let endpoint = host.join(&self.endpoint()).unwrap();
        let request = client.post(endpoint).json(self).build()?;

        tracing::debug!("setting kv store: {:?}", self);

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
                tracing::debug!("set kv store: {}", body);
                Ok(response)
            }
            Err(_) => {
                tracing::error!("failed to set kv store: {}", body);
                
                match serde_json::from_str::<ErrorResponse>(&body) {
                    Ok(response) => Err(Error::Response(response)),
                    Err(e) => Err(Error::Parse(e)),
                }
            }
        }
    }
}
