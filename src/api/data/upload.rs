use anyhow::{bail, Context, Result};
use std::sync::Arc;

use crate::api::{error::ErrorResponse, ApiResult, MecrmRequest, MecrmResponse};

pub struct DataUploadRequest {
    data: Vec<u8>,
}

impl DataUploadRequest {
    pub fn new(data: impl Into<Vec<u8>>) -> DataUploadRequest {
        DataUploadRequest { data: data.into() }
    }
}

impl MecrmRequest<DataUploadResponse> for DataUploadRequest {
    fn endpoint(&self) -> url::Url {
        url::Url::parse("data").unwrap()
    }

    async fn request(
        self,
        client: Arc<reqwest::Client>,
        host: url::Url,
    ) -> Result<ApiResult<DataUploadResponse>> {
        let part = reqwest::multipart::Part::bytes(self.data).file_name("data");
        let multipart = reqwest::multipart::Form::new().part("file", part);

        let response = client
            .post(host.join(&self.endpoint()).unwrap())
            .multipart(multipart)
            .send()
            .await?;

        if let Ok(response) = ErrorResponse::from_response(response).await {
            return Ok(ApiResult::Error(response));
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
