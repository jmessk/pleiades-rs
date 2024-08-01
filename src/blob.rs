use crate::api::{DataDownloadRequest, DataUploadRequest};
use crate::id::Id;
use crate::Client;

use bytes::Bytes;

// #[derive(Debug, typed_builder::TypedBuilder)]
// pub struct Blob {
//     id: Id,
//     data: Option<Bytes>,
// }

#[derive(Debug, typed_builder::TypedBuilder)]
pub struct LocalBlob {
    data: Bytes,
}

impl LocalBlob {
    pub fn data(&self) -> Bytes {
        self.data.clone()
    }

    pub async fn upload(self, client: &Client) -> GlobalBlob {
        let request = DataUploadRequest::builder().data(self.data).build();
        let response = client.request(request).await.unwrap();

        GlobalBlob {
            id: response.data_id.into(),
            data: Some(self.data),
        }
    }
}

#[derive(Debug)]
pub struct GlobalBlob {
    id: Id,
    data: Option<Bytes>,
}

impl GlobalBlob {
    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn data(&mut self) -> Option<Bytes> {
        self.data = None;
        self.data.clone()
    }
}
