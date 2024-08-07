use crate::api::{DataDownloadRequest, DataUploadRequest};
use crate::id::Id;
use crate::Client;

use anyhow::Result;
use bytes::Bytes;
use std::borrow::Cow;
use std::io::Read;

#[derive(Debug, typed_builder::TypedBuilder)]
pub struct LocalBlob {
    client: Client,
    #[builder(setter(into))]
    // data: Cow<'static, [u8]>,
    // #[builder(setter(transform = |data: Cow<'static, [u8]>| data.into_owned()))]
    data: Bytes,
}

impl LocalBlob {
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn input(&mut self, data: Cow<'static, [u8]>) {
        self.data = Bytes::from_static(&data);
    }

    pub async fn upload(self) -> Result<GlobalBlob> {
        let request = DataUploadRequest::builder().data(self.data).build();

        let response = self.client.send(request).await?;

        Ok(GlobalBlob {
            client: self.client,
            id: response.data_id.into(),
        })
    }
}

#[derive(Debug)]
pub struct GlobalBlob {
    client: Client,
    id: Id,
}

impl GlobalBlob {
    pub fn id(&self) -> &Id {
        &self.id
    }

    pub async fn download(self) -> Result<LocalBlob> {
        let request = DataDownloadRequest::builder().data_id(self.id.id()).build();

        let response = self.client.send(request).await?;

        Ok(LocalBlob {
            client: self.client,
            data: response.data.as_ref(),
        })
    }
}
