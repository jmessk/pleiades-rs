use anyhow::{Context, Result};
use reqwest::multipart;
use std::borrow::Cow;
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

pub struct DataUploadBuilder<'a> {
    data: Option<Cow<'a, [u8]>>,
}

impl<'a> DataUploadBuilder<'a> {
    pub fn new() -> DataUploadBuilder<'a> {
        DataUploadBuilder { data: None }
    }

    pub fn build(self) -> Result<DataUploadRequest<'a>> {
        let data = self.data.with_context(|| "data is required")?;

        Ok(DataUploadRequest { data })
    }

    pub fn data(mut self, data: impl Into<Cow<'a, [u8]>>) -> DataUploadBuilder<'a> {
        self.data = Some(data.into());
        self
    }
}

/// Request to upload data
#[derive(Debug)]
pub struct DataUploadRequest<'a> {
    data: Cow<'a, [u8]>,
}

impl<'a> DataUploadRequest<'a> {
    pub fn builder() -> DataUploadBuilder<'a> {
        DataUploadBuilder::new()
    }
}

impl<'a> MecrmRequest for DataUploadRequest<'a> {
    type Response = DataUploadResponse;

    async fn send(&self, client: &Arc<Client>) -> Result<DataUploadResponse> {
        let endpoint = "data";

        let multipart = {
            let part = multipart::Part::bytes(self.data.to_vec()).file_name("data");
            multipart::Form::new().part("file", part)
        };

        let response = client
            .client()
            .post(client.host().join(endpoint).unwrap())
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
        response
            .json()
            .await
            .with_context(|| "Failed to parse response")
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

        let request = DataUploadRequest::builder()
            .data(b"hello world")
            .build()
            .unwrap();

        let response = request.send(&client).await;
        assert!(response.is_ok());

        dbg!(response.unwrap());
    }
}
