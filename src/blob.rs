use crate::api::{DataDownloadRequest, DataUploadRequest};
use crate::id::Id;
use crate::Client;

use anyhow::Result;
use bytes::Bytes;
use std::borrow::Cow;

#[derive(Debug, typed_builder::TypedBuilder)]
pub struct LocalBlob {
    client: Client,
    #[builder(setter(into))]
    data: Cow<'static, [u8]>,
    // #[builder(setter(transform = |data: Cow<'static, [u8]>| data.into_owned()))]
    // data: Bytes,
}

impl LocalBlob {
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub async fn upload(self) -> Result<GlobalBlob> {
        let request = DataUploadRequest::builder().data(self.data.clone()).build();

        let response = self.client.send(request).await?;

        Ok(GlobalBlob {
            client: self.client,
            id: Some(response.data_id.into()),
            data: Some(self.data),
        })
    }
}

#[derive(Debug)]
pub struct GlobalBlob {
    client: Client,
    id: Option<Id>,
    data: Option< Cow<'static, [u8]>>,
}

impl GlobalBlob {
    pub fn id(&self) -> &Id {
        self.id.as_ref().expect("blob has no id")
    }

    pub async fn data(&mut self) -> Result<Bytes> {
        if self.data.is_none() {
            self.download().await?;
        }

        Ok(self.data.as_ref().unwrap().clone())
    }

    pub async fn download(&mut self) -> Result<&GlobalBlob> {
        let request = DataDownloadRequest::builder()
            .data_id(self.id().id())
            .build();

        let response = self.client.send(request).await?;

        self.data = Some(response.data.into());
        Ok(self)
    }
}
