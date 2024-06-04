use anyhow::Result;

use super::{MecrmObject, ObjectBuilder};

pub struct Blob {
    id: String,
    data: Option<Vec<u8>>,
}

impl MecrmObject for Blob {
    fn new() -> impl ObjectBuilder {
        BlobBuilder::new()
    }
}

impl Blob {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn data(&self) -> &[u8] {
        self.data.as_ref().unwrap()
    }
}

struct BlobBuilder {
    id: Option<String>,
    data: Option<Vec<u8>>,
}

impl ObjectBuilder for BlobBuilder {
    fn new() -> BlobBuilder {
        BlobBuilder {
            id: None,
            data: None,
        }
    }

    fn build(self) -> Result<impl MecrmObject> {
        Ok(Blob {
            id: self.id.unwrap(),
            data: self.data,
        })
    }
}

impl BlobBuilder {
    pub fn id(mut self, id: impl Into<String>) -> BlobBuilder {
        self.id = Some(id.into());
        self
    }

    pub fn data(mut self, data: impl Into<Vec<u8>>) -> BlobBuilder {
        self.data = Some(data.into());
        self
    }
}
