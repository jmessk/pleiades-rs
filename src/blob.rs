use anyhow::Result;
use bytes::Bytes;
use std::sync::Arc;

use crate::Id;
use crate::{api, Client, Lambda, Runtime};

#[derive(Debug, Clone)]
pub struct Blob {
    pub(crate) client: Client,
    pub id: Id,
}

impl Blob {
    pub async fn fetch(&self) -> Result<Bytes> {
        let request = api::DataDownloadRequest::builder()
            .data_id(self.id.as_str())
            .build();

        let response = self.client.send(&request).await?;

        Ok(response.data)
    }

    pub async fn into_lambda(self, runtime: impl Into<Runtime>) -> Result<Lambda> {
        let runtime = runtime.into();

        let request = api::LambdaCreateRequest::builder()
            .runtime(runtime.as_str())
            .data_id(self.id.as_str())
            .build();

        let response = self.client.send(&request).await?;

        Ok(crate::Lambda {
            client: self.client.clone(),
            id: response.lambda_id.into(),
            runtime,
            blob: self,
        })
    }
}
