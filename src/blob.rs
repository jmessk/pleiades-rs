pub struct BlobBuilder {
    data: Vec<u8>,
}

impl BlobBuilder {
    pub fn data(data: impl Into<Vec<u8>>) -> BlobBuilder {
        BlobBuilder { data: data.into() }
    }
}

pub struct Blob {
    id: String,
    data: Vec<u8>,
}

impl Blob {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}
