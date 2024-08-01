use crate::id::Id;
use crate::runtime::Runtime;
use crate::blob::GlobalBlob;

#[derive(Debug, typed_builder::TypedBuilder)]
pub struct Lambda {
    pub id: Id,
    pub runtime: Runtime,
    pub blob: GlobalBlob,
}

impl Lambda {
    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }

    pub fn blob(&self) -> &GlobalBlob {
        &self.blob
    }
}
