use anyhow::{bail, Context, Result};
use std::sync::Arc;

use crate::api::{error::MecrmErrorResponse, MecrmRequest, MecrmResponse};

pub struct DataUploadRequest {
    data: Vec<u8>,
}

impl DataUploadRequest {
    const ENDPOINT: &'static str = "data";

    pub fn new(data: impl Into<Vec<u8>>) -> DataUploadRequest {
        DataUploadRequest {
            data: data.into(),
        }
    }
}

impl MecrmRequest for DataUploadRequest {
    async fn request(
        self,
        client: Arc<reqwest::Client>,
        host: url::Url,
    ) -> Result<DataUploadResponse> {
        let multipart = reqwest::multipart::Form::new().part(
            "file",
            reqwest::multipart::Part::bytes(self.data).file_name("data"),
        );

        let response = client
            .post(host.join(Self::ENDPOINT).unwrap())
            .multipart(multipart)
            .send()
            .await?;

        match DataUploadResponse::from_response(response).await {
            Ok(response) => Ok(response),
            Err(_) => bail!("Failed to parse response"),
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
    async fn from_response(response: reqwest::Response) -> Result<DataUploadResponse> {
        response.json().await.context("Failed to parse response")
    }
}
