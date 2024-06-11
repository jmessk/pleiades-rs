use anyhow::{Context, Result};
use reqwest::multipart;
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

/// Request to upload data
pub struct DataUploadRequest {
    data: Vec<u8>,
}

/// Response from uploading data
#[derive(serde::Deserialize)]
pub struct DataUploadResponse {
    pub code: i32,
    pub status: String,
    #[serde(rename = "id")]
    pub data_id: String,
    pub checksum: String,
}

impl DataUploadRequest {
    pub fn new(data: Vec<u8>) -> DataUploadRequest {
        DataUploadRequest { data }
    }
}

impl MecrmRequest for DataUploadRequest {
    type Response = DataUploadResponse;

    async fn send(self, client: Arc<Client>) -> Result<DataUploadResponse> {
        let endpoint = "data";

        let multipart = {
            let part = multipart::Part::bytes(self.data).file_name("data");
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

impl MecrmResponse for DataUploadResponse {
    type Response = DataUploadResponse;

    async fn from_response(response: reqwest::Response) -> Result<DataUploadResponse> {
        response
            .json()
            .await
            .with_context(|| "Failed to parse response")
    }
}
