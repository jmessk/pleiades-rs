use anyhow::Result;
use bytes::Bytes;
use std::borrow::Cow;
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

#[derive(Debug, typed_builder::TypedBuilder)]
pub struct DataDownloadRequest<'a> {
    #[builder(setter(into))]
    data_id: Cow<'a, str>,
}

impl<'a> MecrmRequest for DataDownloadRequest<'a> {
    type Response = DataDownloadResponse;

    fn endpoint(&self, host: &url::Url) -> url::Url {
        host.join(&format!("data/{}/blob", self.data_id)).unwrap()
    }

    async fn send(&self, client: &Arc<Client>) -> Result<DataDownloadResponse> {
        log::debug!("downloading data: {}", self.data_id);

        let response = client
            .client()
            .get(self.endpoint(client.host()))
            .send()
            .await?;

        DataDownloadResponse::from_response(response).await
    }
}

#[derive(Debug)]
pub struct DataDownloadResponse {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_data_download() {
        let client = Client::builder()
            .host("https://mecrm.dolylab.cc/api/v0.5-snapshot/")
            .build();

        let client = Arc::new(client);

        let request = DataDownloadRequest::builder().data_id("1").build();

        let response = request.send(&client).await;
        dbg!(&response);

        assert!(response.is_ok());
    }
}
