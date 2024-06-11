use anyhow::{Context, Result};
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

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

    async fn send(self, client: Arc<Client>) -> Result<DataDownloadResponse> {
        let endpoint = format!("data/{}", self.data_id);

        let response = client
            .client()
            .get(client.host().join(&endpoint).unwrap())
            .send()
            .await?;

        DataDownloadResponse::from_response(response).await
    }
}

pub struct DataDownloadResponse {
    pub data: Vec<u8>,
}

impl MecrmResponse for DataDownloadResponse {
    type Response = DataDownloadResponse;

    async fn from_response(response: reqwest::Response) -> Result<DataDownloadResponse> {
        Ok(DataDownloadResponse {
            data: response
                .bytes()
                .await
                .with_context(|| "Failed to parse response")?
                .to_vec(),
        })
    }
}
