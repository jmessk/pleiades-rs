use anyhow::Result;
use bytes::Bytes;
use std::borrow::Cow;

use crate::api::{MecrmRequest, MecrmResponse};

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

impl<'a> MecrmRequest for DataDownloadRequest<'a> {
    type Response = DataDownloadResponse;

    fn endpoint(&self) -> String {
        format!("data/{}/blob", self.data_id)
    }

    async fn send(
        &self,
        client: &reqwest::Client,
        host: &url::Url,
    ) -> Result<DataDownloadResponse> {
        log::debug!("downloading data: {}", self.data_id);

        let endpoint = host.join(&self.endpoint()).unwrap();
        let response = client.get(endpoint).send().await?;

        DataDownloadResponse::from_response(response).await
    }
}

/// Response from downloading blob data
#[derive(Debug)]
pub struct DataDownloadResponse {
    /// downloaded data
    pub data: Bytes,
}

impl MecrmResponse for DataDownloadResponse {
    type Response = DataDownloadResponse;

    async fn from_response(response: reqwest::Response) -> Result<DataDownloadResponse> {
        match response.headers().get("content-type") {
            Some(content_type) => match content_type.to_str()? {
                // application/octet-stream is the blob data
                "application/octet-stream" => {
                    let data = response.bytes().await?;

                    log::info!("data downloaded");
                    log::debug!("downloaded {} bytes", data.len());

                    Ok(DataDownloadResponse { data })
                }

                // application/json is the error message
                _ => {
                    let error = response.text().await?;
                    log::error!("failed to download data: {}", error);

                    anyhow::bail!("failed to download data")
                }
            },
            None => {
                let error = response.text().await?;
                log::error!("failed to download data: {}", error);

                anyhow::bail!("failed to download data")
            }
        }
    }
}
