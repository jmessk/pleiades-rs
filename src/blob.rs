pub struct Blob {
    id: Option<String>,
    data: Option<Vec<u8>>,
}

impl Blob {
    pub fn new() -> BlobBuilder {
        BlobBuilder::new()
    }

    pub fn id(&self) -> &str {
        self.id.as_deref()
    }

    pub fn data(&self) -> &[u8] {
        self.data
    }
}

pub struct BlobBuilder {
    inner: Blob,
}

impl BlobBuilder {
    pub fn new() -> BlobBuilder {
        BlobBuilder {
            inner: Blob {
                id: None,
                data: None,
            }
        }
    }

    pub fn build(self) -> Blob {
        Blob {
            id: self.id.expect("id is required"),
            data: self.data.expect("data is required"),
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> BlobBuilder {
        self.inner.id = Some(id.into());
        self
    }

    pub fn data(mut self, data: impl Into<Vec<u8>>) -> BlobBuilder {
        self.inner.data = Some(data.into());
        self
    }
}
