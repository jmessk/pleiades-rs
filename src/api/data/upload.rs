use anyhow::{Context, Result};
use bytes::Bytes;
use reqwest::multipart;
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Handler;

pub struct DataUploadBuilder {
    handler: Arc<Handler>,
    data: Option<Bytes>,
}

impl DataUploadBuilder {
    pub fn new(handler: Arc<Handler>) -> DataUploadBuilder {
        DataUploadBuilder {
            handler,
            data: None,
        }
    }

    pub fn build(self) -> Result<DataUploadRequest> {
        let data = self.data.with_context(|| "data is required")?;

        Ok(DataUploadRequest {
            handler: self.handler,
            data,
        })
    }

    pub fn data(mut self, data: Bytes) -> DataUploadBuilder {
        self.data = Some(data);
        self
    }
}

/// Request to upload data
#[derive(Debug)]
pub struct DataUploadRequest {
    handler: Arc<Handler>,
    data: Bytes,
}

impl DataUploadRequest {
    pub fn builder(client: Arc<Handler>) -> DataUploadBuilder {
        DataUploadBuilder::new(client)
    }
}

impl MecrmRequest for DataUploadRequest {
    type Response = DataUploadResponse;

    async fn send(&self) -> Result<DataUploadResponse> {
        let endpoint = "data";

        let multipart = {
            let part = multipart::Part::bytes(self.data.to_vec()).file_name("data");
            multipart::Form::new().part("file", part)
        };

        let response = self
            .handler
            .client()
            .post(self.handler.host().join(endpoint).unwrap())
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
    use crate::Handler;

    #[tokio::test]
    async fn test_data_upload() {
        let handler = Handler::builder()
            .host("https://mecrm.dolylab.cc/api/v0.5-snapshot/")
            .build()
            .unwrap();

        let handler = Arc::new(handler);

        let request = DataUploadRequest::builder(handler.clone())
            .data(Bytes::from_static(b"hello world"))
            .build()
            .unwrap();

        let response = request.send().await;
        assert!(response.is_ok());

        dbg!(response.unwrap());
    }
}
