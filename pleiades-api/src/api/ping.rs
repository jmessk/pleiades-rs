use std::borrow::Cow;

use crate::api::{error, CoreRequest, CoreResponse, Error, Result};

pub struct Request;

impl CoreRequest for Request {
    type Response = Response;

    fn endpoint(&self) -> Cow<'static, str> {
        "ping".into()
    }

    async fn send(&self, client: &reqwest::Client, host: &url::Url) -> Result<Response> {
        let endpoint = host.join(&self.endpoint()).unwrap();
        let request = client.get(endpoint).build()?;

        tracing::debug!("sending ping");

        let response = client.execute(request).await?;
        Response::from_response(response).await
    }
}

/// Response from creating a lambda
#[derive(serde::Deserialize, Debug)]
pub struct Response {
    pub code: u32,
    pub status: String,
    pub message: String,
}

impl CoreResponse for Response {
    type Response = Response;

    async fn from_response(response: reqwest::Response) -> Result<Response> {
        let body = response.text().await?;

        match serde_json::from_str::<Response>(&body) {
            Ok(response) => {
                tracing::debug!("receive ping: {}", body);
                Ok(response)
            }
            Err(_) => {
                tracing::error!("failed to receive ping: {}", body);

                match serde_json::from_str::<error::Response>(&body) {
                    Ok(response) => Err(Error::Response(response)),
                    Err(e) => Err(Error::Parse(e)),
                }
            }
        }
    }
}
