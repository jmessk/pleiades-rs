use anyhow::{bail, Context, Result};
use reqwest::multipart;
use std::sync::Arc;

use crate::api::{error::ErrorResponse, MecrmRequest, MecrmResponse};

pub struct DataUploadRequest {
    data: Vec<u8>,
}

impl DataUploadRequest {
    pub fn new() -> DataUploadRequestBuilder {
        DataUploadRequestBuilder::new()
    }

    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
    }
}

pub struct DataUploadRequestBuilder {
    data: Option<Vec<u8>>,
}

impl DataUploadRequestBuilder {
    pub fn new() -> DataUploadRequestBuilder {
        DataUploadRequestBuilder { data: None }
    }

    pub fn data(mut self, data: impl Into<Vec<u8>>) -> DataUploadRequestBuilder {
        self.data = Some(data.into());
        self
    }

    pub fn build(self) -> Result<DataUploadRequest> {
        Ok(DataUploadRequest {
            data: self.data.unwrap(),
        })
    }
}

impl MecrmRequest for DataUploadRequest {
    type Response = DataUploadResponse;

    async fn request(
        self,
        client: Arc<reqwest::Client>,
        host: url::Url,
    ) -> Result<DataUploadResponse> {
        let endpoint = "data";

        let multipart = {
            let part = multipart::Part::bytes(self.data).file_name("data");
            multipart::Form::new().part("file", part)
        };

        let response = client
            .post(host.join(endpoint).unwrap())
            .multipart(multipart)
            .send()
            .await?;

        if let Ok(response) = DataUploadResponse::from_response(response).await {
            Ok(response)
        } else {
            anyhow::bail!("Failed to upload data")
        }
    }
}

#[derive(serde::Deserialize)]
pub struct DataUploadResponse {
    code: u32,
    status: String,
    data_id: String,
    checksum: String,
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
