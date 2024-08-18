use std::borrow::Cow;

use crate::api::{Error, ErrorResponse, Request, Response, Result};

///
#[derive(serde::Serialize, Debug, typed_builder::TypedBuilder)]
pub struct Create<'a> {
    ///
    #[builder(setter(into))]
    pub consistency: Cow<'a, str>,
}

impl<'a> Request for Create<'a> {
    type Response = CreateResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        "kv".into()
    }

    async fn send(&self, client: &reqwest::Client, host: &url::Url) -> Result<CreateResponse> {
        let endpoint = host.join(&self.endpoint()).unwrap();
        let request = client.post(endpoint).json(self).build()?;

        tracing::debug!("creating namespace: {:?}", self);

        let response = client.execute(request).await?;
        CreateResponse::from_response(response).await
    }
}

/// Response from creating a lambda
#[derive(serde::Deserialize, Debug)]
pub struct CreateResponse {
    pub code: u32,
    // pub status: String,

    /// created lambda ID
    #[serde(rename = "id")]
    pub namespace_id: String,
}

impl Response for CreateResponse {
    type Response = CreateResponse;

    async fn from_response(response: reqwest::Response) -> Result<CreateResponse> {
        let body = response.text().await?;

        match serde_json::from_str::<CreateResponse>(&body) {
            Ok(response) => {
                tracing::debug!("namespace created: {}", body);
                Ok(response)
            }
            Err(_) => {
                tracing::error!("failed to create kv namespace: {}", body);

                match serde_json::from_str::<ErrorResponse>(&body) {
                    Ok(response) => Err(Error::Response(response)),
                    Err(e) => Err(Error::Parse(e)),
                }
            }
        }
    }
}
