use anyhow::Result;
use bytes::Bytes;

use crate::Id;
use crate::{api, Client, Lambda, Runtime};

pub struct Selector {
    pub(crate) client: Client,
}

impl Selector {
    #[allow(clippy::new_ret_no_self, clippy::wrong_self_convention)]
    pub async fn new(self, data: impl Into<Bytes>) -> anyhow::Result<Blob> {
        let request = api::DataUploadRequest { data: data.into() };
        let response = self.client.send(&request).await?;

        Ok(Blob {
            client: self.client,
            id: response.data_id.into(),
        })
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn from_id(self, id: impl Into<Id>) -> Blob {
        Blob {
            client: self.client,
            id: id.into(),
        }
    }
}

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
