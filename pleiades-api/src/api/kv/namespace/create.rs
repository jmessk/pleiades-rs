use std::borrow::Cow;

use crate::api::{Error, CoreRequest, CoreResponse, Result};

///
#[derive(serde::Serialize, Debug, typed_builder::TypedBuilder)]
pub struct Request<'a> {
    ///
    #[builder(setter(into))]
    pub consistency: Cow<'a, str>,
}

impl<'a> CoreRequest for Request<'a> {
    type Response = Response;

    fn endpoint(&self) -> Cow<'static, str> {
        "kv".into()
    }

    async fn send(&self, client: &reqwest::Client, host: &url::Url) -> Result<Response> {
        let endpoint = host.join(&self.endpoint()).unwrap();
        let request = client.post(endpoint).json(self).build()?;

        tracing::debug!("creating namespace: {:?}", self);

        let response = client.execute(request).await?;
        Response::from_response(response).await
    }
}

/// Response from creating a lambda
#[derive(serde::Deserialize, Debug)]
pub struct Response {
    pub code: u32,
    // pub status: String,

    /// created lambda ID
    #[serde(rename = "id")]
    pub namespace_id: String,
}

impl CoreResponse for Response {
    type Response = Response;

    async fn from_response(response: reqwest::Response) -> Result<Response> {
        let body = response.text().await?;

        match serde_json::from_str::<Response>(&body) {
            Ok(response) => {
                tracing::debug!("namespace created: {}", body);
                Ok(response)
            }
            Err(_) => {
                tracing::error!("failed to create kv namespace: {}", body);
                Err(Error::parse(&body))
            }
        }
    }
}
