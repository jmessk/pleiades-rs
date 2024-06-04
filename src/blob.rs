use anyhow::Result;

use super::{MecrmObject, ObjectBuilder};

pub struct Blob {
    id: Option<String>,
    data: Option<Vec<u8>>,
}

impl MecrmObject for Blob {
    fn new() -> impl ObjectBuilder {
        BlobBuilder::new()
    }
}

impl Blob {
    fn id(&self) -> &str {
        self.id.as_deref().unwrap()
    }

    pub fn data(&self) -> &[u8] {
        self.data.as_deref().unwrap()
    }
}

struct BlobBuilder {
    inner: Blob,
}

impl ObjectBuilder for BlobBuilder {
    fn new() -> BlobBuilder {
        BlobBuilder {
            inner: Blob {
                id: None,
                data: None,
            },
        }
    }

    fn build(self) -> Result<impl MecrmObject> {
        Ok(Blob {
            id: Some(self.inner.id.unwrap()),
            data: Some(self.inner.data.unwrap()),
        })
    }
}

impl BlobBuilder {
    pub fn id(mut self, id: impl Into<String>) -> BlobBuilder {
        self.inner.id = Some(id.into());
        self
    }

    pub fn data(mut self, data: impl Into<Vec<u8>>) -> BlobBuilder {
        self.inner.data = Some(data.into());
        self
    }
}
