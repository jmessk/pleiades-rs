use bytes::Bytes;
use std::borrow::Cow;

use crate::api::{ApiError, ApiRequest, ApiResponse, Result, error};

/// Request to download byte data
///
/// # Example
///
/// ```rust
/// use mecrs::api::data::download::DataDownloadRequest;
/// use bytes::Bytes;
///
/// let request = DataDownloadRequest::builder()
///     .data_id("1")
///     .build();
///
/// let response = request.send(&client).await?;
/// let data: Bytes = response.data;
/// ```
#[derive(Debug, typed_builder::TypedBuilder)]
pub struct Request<'a> {
    /// blob data ID to download
    #[builder(setter(into))]
    pub data_id: Cow<'a, str>,
}

impl<'a> ApiRequest for Request<'a> {
    type Response = Response;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("data/{}/blob", self.data_id).into()
    }

    async fn send(
        &self,
        client: &reqwest::Client,
        host: &url::Url,
    ) -> Result<Response> {
        let endpoint = host.join(&self.endpoint()).unwrap();
        let request = client.get(endpoint).build()?;

        tracing::debug!("downloading data: {:?}", self.data_id);

        let response = client.execute(request).await?;
        Response::from_response(response).await
    }
}

/// Response from downloading blob data
#[derive(Debug)]
pub struct Response {
    /// downloaded data
    pub data: Bytes,
}

impl ApiResponse for Response {
    type Response = Response;

    async fn from_response(response: reqwest::Response) -> Result<Response> {
        let content_type = response.headers().get("content-type");

        if content_type.is_none() {
            let error = response.bytes().await?;
            tracing::error!("failed to download data: {:?}", error);

            return Err(ApiError::Other(anyhow::anyhow!("failed to read content type")));
        }

        match content_type.unwrap().to_str().unwrap() {
            // application/octet-stream is the blob data
            "application/octet-stream" => {
                let data = response.bytes().await?;
                tracing::debug!("downloaded {} bytes", data.len());

                Ok(Response { data })
            }

            "application/json" => {
                let body = response.text().await?;
                tracing::error!("failed to download data: {}", body);
                let json = serde_json::from_str::<error::Response>(&body);

                match json {
                    Ok(response) => Err(ApiError::Response(response)),
                    Err(e) => Err(ApiError::Parse(e)),
                }
            }

            // other content types are not supported
            _ => {
                let error = response.bytes().await?;
                tracing::error!("failed to download data: {:?}", error);

                Err(ApiError::Other(anyhow::anyhow!("failed to download data")))
            }
        }
    }
}
