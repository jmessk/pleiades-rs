use anyhow::Result;
use bytes::Bytes;

use crate::api::{DataDownloadRequest, DataUploadRequest};
use crate::Client;
use crate::Id;

#[derive(Debug)]
pub struct Blob {
    pub id: Id,
}

impl Blob {
    pub fn from_id(id: impl Into<Id>) -> Self {
        Self { id: id.into() }
    }
}

impl From<Blob> for Id {
    fn from(blob: Blob) -> Self {
        blob.id
    }
}
