use anyhow::Result;
use bytes::Bytes;

use crate::api::{DataDownloadRequest, DataUploadRequest};
use crate::id::Id;
use crate::Client;

#[derive(Debug)]
pub struct BlobBuilder {
    client: Client,
    data: Option<Bytes>,
}

impl BlobBuilder {
    pub fn new(client: Client) -> Self {
        Self { client, data: None }
    }

    pub fn data(mut self, data: Bytes) -> Self {
        self.data = Some(data);
        self
    }

    pub async fn build(self) -> Result<Blob> {
        let data = self.data.expect("data is required");

        let request = DataUploadRequest::builder().data(data.clone()).build();
        let response = self.client.send(request).await?;

        Ok(Blob {
            id: response.data_id.into(),
            data: data,
        })
    }
}

#[derive(Debug)]
pub struct Blob {
    id: Id,
    data: Bytes,
}

impl Blob {
    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn data(&self) -> Bytes {
        self.data.clone()
    }
}

#[derive(Debug)]
pub struct RemoteBlob {
    client: Client,
    id: Id,
}
