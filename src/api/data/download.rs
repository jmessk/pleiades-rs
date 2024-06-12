use anyhow::{Context, Result};
use bytes::Bytes;
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Handler;

pub struct DataDownloadBuilder {
    handler: Arc<Handler>,
    data_id: Option<String>,
}

impl DataDownloadBuilder {
    pub fn new(client: Arc<Handler>) -> DataDownloadBuilder {
        DataDownloadBuilder {
            handler: client,
            data_id: None,
        }
    }

    pub fn build(self) -> Result<DataDownloadRequest> {
        let data_id = self.data_id.with_context(|| "data_id is required")?;

        Ok(DataDownloadRequest {
            handler: self.handler,
            data_id,
        })
    }

    pub fn data_id(mut self, data_id: impl Into<String>) -> DataDownloadBuilder {
        self.data_id = Some(data_id.into());
        self
    }
}

#[derive(Debug)]
pub struct DataDownloadRequest {
    handler: Arc<Handler>,
    data_id: String,
}

impl DataDownloadRequest {
    pub fn builder(client: Arc<Handler>) -> DataDownloadBuilder {
        DataDownloadBuilder::new(client)
    }
}

impl MecrmRequest for DataDownloadRequest {
    type Response = DataDownloadResponse;

    async fn send(&self) -> Result<DataDownloadResponse> {
        let endpoint = format!("data/{}", self.data_id);

        let response = self
            .handler
            .client()
            .get(self.handler.host().join(&endpoint).unwrap())
            .send()
            .await?;

        DataDownloadResponse::from_response(response).await
    }
}

#[derive(Debug)]
pub struct DataDownloadResponse {
    pub data: Bytes,
}

impl MecrmResponse for DataDownloadResponse {
    type Response = DataDownloadResponse;

    async fn from_response(response: reqwest::Response) -> Result<DataDownloadResponse> {
        Ok(DataDownloadResponse {
            data: response.bytes().await?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Handler;

    #[tokio::test]
    async fn test_data_download() {
        let handler = Handler::builder()
            .host("https://mecrm.dolylab.cc/api/v0.5-snapshot/")
            .build()
            .unwrap();

        let handler = Arc::new(handler);

        let request = DataDownloadRequest::builder(handler.clone())
            .data_id("0")
            .build()
            .unwrap();

        let response = request.send().await;
        assert!(response.is_ok());
    }
}
