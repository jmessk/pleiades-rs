use anyhow::Result;
use reqwest::multipart;
use std::borrow::Cow;
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

/// Request to upload byte data
/// 
/// # Example
/// 
/// ```rust
/// use mecrs::api::data::upload::DataUploadRequest;
/// 
/// let request = DataUploadRequest::builder()
///    .data(b"hello world")
///    .build();
/// 
/// let response = request.send(&client).await?;
/// let data_id = response.data_id;
/// ```
#[derive(Debug, typed_builder::TypedBuilder)]
pub struct DataUploadRequest<'a> {
    /// byte data to upload
    #[builder(setter(into))]
    data: Cow<'a, [u8]>,
}

impl<'a> MecrmRequest for DataUploadRequest<'a> {
    type Response = DataUploadResponse;

    fn endpoint(&self, host: &url::Url) -> url::Url {
        host.join("data").unwrap()
    }

    async fn send(&self, client: &Arc<Client>) -> Result<DataUploadResponse> {
        log::debug!("uploading data: {} bytes", self.data.len());

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

    /// uploaded blob data ID
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
                anyhow::bail!("failed to parse response: {}", e)
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
            .build();

        let client = Arc::new(client);

        let request = DataUploadRequest::builder().data(b"hello world").build();

        let response = request.send(&client).await;
        assert!(response.is_ok());

        dbg!(response.unwrap());
    }
}
