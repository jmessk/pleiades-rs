use anyhow::{bail, Context as _, Result};

use super::{MecrmObject, ObjectBuilder, Client};

pub struct Blob {
    id: String,
    data: Option<Vec<u8>>,
}

impl MecrmObject for Blob {
    type Builder = BlobBuilder;
}

impl Blob {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub async fn data(&self) -> Result<&Vec<u8>> {
        self.data.as_ref().context("Blob data is not set")
    }
}

pub struct BlobBuilder {
    client: Client,
    id: Option<String>,
    data: Option<Vec<u8>>,
}

impl ObjectBuilder for BlobBuilder {
    type Output = Blob;
}

impl BlobBuilder {
    pub fn id(mut self, id: impl Into<String>) -> Blob {
        Blob {
            id: id.into(),
            data: self.data,
        }
    }

    pub fn data(mut self, data: impl Into<Vec<u8>>) -> BlobBuilder {
        self.data = Some(data.into());
        self
    }
}
