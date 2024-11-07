use pleiades_api::api;

use crate::{
    blob::Blob,
    client::Client,
    feature::{id::Id, runtime::Runtime},
    job::Job,
};

pub struct Selector<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> Selector<'a> {
    #[allow(clippy::wrong_self_convention)]
    pub async fn from_id(self, id: impl Into<Id>) -> anyhow::Result<Lambda> {
        let _ = id;
        let _ = self.client;
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Lambda {
    pub(crate) client: Client,
    pub id: Id,
    pub runtime: Runtime,
    pub code: Blob,
}

impl Lambda {
    pub async fn invoke(&self, input: Blob, tags: Option<&[&str]>) -> anyhow::Result<Job> {
        let request = api::job::create::Request {
            lambda_id: self.id.as_str().into(),
            data_id: input.id.as_str().into(),
            tags: tags.unwrap_or_default(),
        };
        let response = self.client.inner.call_api(&request).await?;

        Ok(Job {
            client: self.client.clone(),
            id: response.job_id.into(),
            lambda: self.clone(),
            input,
        })
    }
}
