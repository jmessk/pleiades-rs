use crate::{api, blob::Blob, job::Job, Client, Id, Runtime};

pub struct Selector {
    pub(crate) client: Client,
}

impl Selector {
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
    pub blob: Blob,
}

impl Lambda {
    pub async fn invoke(&self, input: Blob) -> anyhow::Result<Job> {
        let request = api::JobCreateRequest::builder()
            .lambda_id(self.id.as_str())
            .data_id(input.id.as_str())
            .build();

        let response = self.client.send(&request).await?;

        Ok(Job {
            client: self.client.clone(),
            id: response.job_id.into(),
            lambda: self.clone(),
            input,
        })
    }
}
