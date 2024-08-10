use std::borrow::Cow;

use crate::{blob::Blob, job::Job, Client, Id, Runtime};

#[derive(Debug)]
pub struct Lambda {
    pub client: Client,
    pub id: Id,
    pub runtime: Runtime,
    pub blob: Blob,
}

impl Lambda {
    pub async fn invoke
    // <T, U>
    (&self, input: Blob, 
        // tags:
        // T
        // impl IntoIterator<Item = impl Into<Cow<'static, str>>>
    ) -> anyhow::Result<Job>
    // where
    //     T: IntoIterator<Item = U>,
    //     U: Into<Cow<'static, str>>,
    {
        todo!()
    }
}
