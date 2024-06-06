use anyhow::{bail, Context, Result};
use std::sync::Arc;

use crate::api::{error::ErrorResponse, MecrmRequest, MecrmResponse};

pub struct DataDownloadRequest {
    data_id: String,
}

impl DataDownloadRequest {
    const ENDPOINT: &'static str = "data";

    pub fn new(data_id: impl Into<String>) -> DataDownloadRequest {
        DataDownloadRequest {
            data_id: data_id.into(),
        }
    }
}

impl MecrmRequest for DataDownloadRequest {
    async fn request(
        self,
        client: Arc<reqwest::Client>,
        host: url::Url,
    ) -> Result<DataDownloadResponse> {
        let response = client
            .post(
                host.join(Self::ENDPOINT)
                    .unwrap()
                    .join(&self.data_id)
                    .unwrap(),
            )
            .send()
            .await?;

        match DataDownloadResponse::from_response(response).await {
            Ok(response) => Ok(response),
            Err(_) => bail!("Failed to parse response"),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct DataDownloadResponse {
    code: u32,
    status: String,
    data_id: String,
    checksum: String,
}

impl MecrmResponse for DataDownloadResponse {
    async fn from_response(response: reqwest::Response) -> Result<DataDownloadResponse> {
        response.json().await.context("Failed to parse response")
    }
}
