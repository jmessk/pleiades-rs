use bytes::Bytes;
use std::borrow::Cow;

use crate::api::{Error, ErrorResponse, Request, Response, Result};

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
pub struct DataDownloadRequest<'a> {
    /// blob data ID to download
    #[builder(setter(into))]
    data_id: Cow<'a, str>,
}

impl<'a> Request for DataDownloadRequest<'a> {
    type Response = DataDownloadResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        format!("data/{}/blob", self.data_id).into()
    }

    async fn send(
        &self,
        client: &reqwest::Client,
        host: &url::Url,
    ) -> Result<DataDownloadResponse> {
        let endpoint = host.join(&self.endpoint()).unwrap();
        let request = client.get(endpoint).build()?;

        log::debug!("downloading data: {:?}", self.data_id);

        let response = client.execute(request).await?;
        DataDownloadResponse::from_response(response).await
    }
}

/// Response from downloading blob data
#[derive(Debug)]
pub struct DataDownloadResponse {
    /// downloaded data
    pub data: Bytes,
}

impl Response for DataDownloadResponse {
    type Response = DataDownloadResponse;

    async fn from_response(response: reqwest::Response) -> Result<DataDownloadResponse> {
        let content_type = response.headers().get("content-type");

        if content_type.is_none() {
            let error = response.text().await?;
            log::error!("failed to download data: {}", error);

            return Err(Error::Other(anyhow::anyhow!("failed to read content type")));
        }

        match content_type.unwrap().to_str().unwrap() {
            // application/octet-stream is the blob data
            "application/octet-stream" => {
                let data = response.bytes().await?;

                log::info!("data downloaded");
                log::debug!("downloaded {} bytes", data.len());

                Ok(DataDownloadResponse { data })
            }

            // application/json is the error message
            "application/json" => match response.json::<ErrorResponse>().await {
                Ok(response) => {
                    log::error!("failed to download data: {}", response);
                    Err(Error::Response(response))
                }
                Err(e) => {
                    log::error!("failed to download data: {}", e);
                    Err(Error::Request(e))
                }
            },

            // other content types are not supported
            _ => {
                let error = response.text().await?;
                log::error!("failed to download data: {}", error);

                Err(Error::Other(anyhow::anyhow!("failed to download data")))
            }
        }
    }
}
