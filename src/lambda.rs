use std::borrow::Cow;

use crate::{api, blob::Blob, job::Job, Client, Id, Runtime};

#[derive(Debug, Clone)]
pub struct Lambda {
    pub client: Client,
    pub id: Id,
    pub runtime: Runtime,
    pub blob: Blob,
}

impl Lambda {
    pub async fn invoke(
        &self,
        input: Blob,
        // tags:
        // T
        // impl IntoIterator<Item = impl Into<Cow<'static, str>>>
    ) -> anyhow::Result<Job>
// where
    //     T: IntoIterator<Item = U>,
    //     U: Into<Cow<'static, str>>,
    {
        let request = api::JobCreateRequest::builder()
            .lambda_id(self.id.as_str())
            .data_id(input.id.as_str())
            .build();

        let response = self.client.send(request).await?;

        Ok(Job {
            client: self.client.clone(),
            id: response.job_id.into(),
            lambda: self.clone(),
            input,
        })
    }
}
