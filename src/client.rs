use anyhow::Result;
use bytes::Bytes;
use std::sync::Arc;

use crate::{api, job::Job, Blob, Id, Lambda, Runtime, Worker};

#[derive(Debug, Clone, typed_builder::TypedBuilder)]
pub struct Client {
    #[builder(default = reqwest::Client::new())]
    client: reqwest::Client,

    #[builder(setter(transform = |url: impl reqwest::IntoUrl| Arc::new(url.into_url().expect("invalid host"))))]
    host: Arc<url::Url>,
}

impl Client {
    // pub fn builder() -> ClientBuilder {
    //     ClientBuilder::new()
    // }

    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    pub fn host(&self) -> &url::Url {
        &self.host
    }

    pub async fn send<T: api::Request>(&self, request: &T) -> api::Result<T::Response> {
        request.send(&self.client, &self.host).await
    }

    pub async fn new_blob(&self, data: impl Into<Bytes>) -> Result<Blob> {
        let request = api::DataUploadRequest { data: data.into() };
        let response = self.send(&request).await?;

        Ok(Blob {
            client: self.clone(),
            id: response.data_id.into(),
        })
    }

    pub fn blob_from_id(&self, id: impl Into<Id>) -> Blob {
        Blob {
            client: self.clone(),
            id: id.into(),
        }
    }

    pub async fn lambda_from_id(&self, id: impl Into<Id>) -> Result<Lambda> {
        todo!()
    }

    pub async fn job_from_id(&self, id: impl Into<Id>) -> Result<Job> {
        todo!()
    }

    // pub async fn new_worker<T, U>(&self, runtimes: T) -> Result<Worker>
    // where
    //     T: IntoIterator<Item = U>,
    //     U: Into<Runtime>,
    // {
    //     let request = api::WorkerRegisterRequest::builder()
    //         .runtimes(runtimes)
    //         .build();

    //     todo!()
    // }
}
