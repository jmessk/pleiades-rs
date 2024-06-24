use anyhow::{bail, Result};
use reqwest::multipart;
use std::borrow::Cow;
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

/// Request to upload data
#[derive(Debug, typed_builder::TypedBuilder)]
pub struct DataUploadRequest<'a> {
    #[builder(setter(into))]
    data: Cow<'a, [u8]>,
}

impl<'a> DataUploadRequest<'a> {}

impl<'a> MecrmRequest for DataUploadRequest<'a> {
    type Response = DataUploadResponse;

    fn endpoint(&self, host: &url::Url) -> url::Url {
        host.join("data").unwrap()
    }

    async fn send(&self, client: &Arc<Client>) -> Result<DataUploadResponse> {
        let multipart = {
            let part = multipart::Part::bytes(self.data.to_vec()).file_name("data");
            multipart::Form::new().part("file", part)
        };

        let response = client
            .client()
            .post(self.endpoint(client.host()))
            .multipart(multipart)
            .send()
            .await?;

        DataUploadResponse::from_response(response).await
    }
}

/// Response from uploading data
#[derive(serde::Deserialize, Debug)]
pub struct DataUploadResponse {
    pub code: u32,
    pub status: String,
    #[serde(rename = "id")]
    pub data_id: String,
    pub checksum: String,
}

impl MecrmResponse for DataUploadResponse {
    type Response = DataUploadResponse;

    async fn from_response(response: reqwest::Response) -> Result<DataUploadResponse> {
        let body = response.text().await?;

        match serde_json::from_str(&body) {
            Ok(response) => {
                log::info!("data uploaded");
                log::debug!("data uploaded: {}", body);
                Ok(response)
            }
            Err(e) => {
                log::error!("failed to parse response: {}", body);
                bail!("failed to parse response: {}", e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Client;

    #[tokio::test]
    async fn test_data_upload() {
        let client = Client::builder()
            .host("https://mecrm.dolylab.cc/api/v0.5-snapshot/")
            .build()
            .unwrap();

        let client = Arc::new(client);

        let request = DataUploadRequest::builder().data(b"hello world").build();

        let response = request.send(&client).await;
        assert!(response.is_ok());

        dbg!(response.unwrap());
    }
}
