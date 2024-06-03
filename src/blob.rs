pub struct Blob {
    id: String,
    data: Vec<u8>,
}

impl Blob {
    pub fn new() -> BlobBuilder {
        BlobBuilder::new()
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

pub struct BlobBuilder {
    id: Option<String>,
    data: Option<Vec<u8>>,
}

impl BlobBuilder {
    pub fn new() -> BlobBuilder {
        BlobBuilder {
            id: None,
            data: None,
        }
    }

    pub fn build(self) -> Blob {
        Blob {
            id: self.id.expect("id is required"),
            data: self.data.expect("data is required"),
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> BlobBuilder {
        self.id = Some(id.into());
        self
    }

    pub fn data(mut self, data: impl Into<Vec<u8>>) -> BlobBuilder {
        self.data = Some(data.into());
        self
    }
}
