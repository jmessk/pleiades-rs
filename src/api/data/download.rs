use anyhow::{bail, Context, Result};
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};

pub struct DataDownloadRequest {
    data_id: String,
}

impl DataDownloadRequest {
    pub fn new(data_id: impl Into<String>) -> DataDownloadRequest {
        DataDownloadRequest {
            data_id: data_id.into(),
        }
    }
}

impl MecrmRequest for DataDownloadRequest {
    type Response = DataDownloadResponse;

    async fn request(
        self,
        client: Arc<reqwest::Client>,
        host: url::Url,
    ) -> Result<DataDownloadResponse> {
        let endpoint = "data";
        let response = client
            .post(host.join(endpoint).unwrap().join(&self.data_id).unwrap())
            .send()
            .await?;

        DataDownloadResponse::from_response(response).await
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
    type Response = DataDownloadResponse;

    async fn from_response(response: reqwest::Response) -> Result<DataDownloadResponse> {
        response
            .json()
            .await
            .with_context(|| "Failed to parse response")
    }
}
