use crate::id::Id;

use bytes::Bytes;

#[derive(Debug, typed_builder::TypedBuilder)]
pub struct LocalBlob {
    data: Bytes,
}

impl LocalBlob {
    pub fn data(&self) -> Bytes {
        self.data.clone()
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

    pub fn data(&self) -> Option<Bytes> {
        self.data.clone()
    }
}
