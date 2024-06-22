use anyhow::{Context, Result};
use bytes::Bytes;
use std::borrow::Cow;
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

pub struct DataDownloadBuilder<'a> {
    data_id: Option<Cow<'a, str>>,
}

impl<'a> DataDownloadBuilder<'a> {
    pub fn new() -> DataDownloadBuilder<'a> {
        DataDownloadBuilder { data_id: None }
    }

    pub fn build(self) -> Result<DataDownloadRequest<'a>> {
        let data_id = self.data_id.with_context(|| "data_id is required")?;

        Ok(DataDownloadRequest { data_id })
    }

    pub fn data_id(mut self, data_id: impl Into<Cow<'a, str>>) -> DataDownloadBuilder<'a> {
        self.data_id = Some(data_id.into());
        self
    }
}

#[derive(Debug)]
pub struct DataDownloadRequest<'a> {
    data_id: Cow<'a, str>,
}

impl<'a> DataDownloadRequest<'a> {
    pub fn builder() -> DataDownloadBuilder<'a> {
        DataDownloadBuilder::new()
    }
}

impl<'a> MecrmRequest for DataDownloadRequest<'a> {
    type Response = DataDownloadResponse;

    async fn send(&self, client: &Arc<Client>) -> Result<DataDownloadResponse> {
        let endpoint = format!("data/{}/blob", self.data_id);

        let response = client
            .client()
            .get(client.host().join(&endpoint).unwrap())
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

    #[tokio::test]
    async fn test_data_download() {
        let client = Client::builder()
            .host("https://mecrm.dolylab.cc/api/v0.5-snapshot/")
            .build()
            .unwrap();

        let client = Arc::new(client);

        let request = DataDownloadRequest::builder().data_id("0").build().unwrap();

        let response = request.send(&client).await;
        assert!(response.is_ok());
    }
}
