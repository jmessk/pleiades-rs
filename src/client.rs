use std::sync::Arc;

use anyhow::{Context, Result};
use bytes::Bytes;
use reqwest::IntoUrl;

use crate::{api, job::Job, Blob, Id, Lambda};

#[derive(Debug, typed_builder::TypedBuilder)]
struct Inner {
    #[builder(default = reqwest::Client::new())]
    client: reqwest::Client,

    #[builder(setter(into))]
    host: url::Url,
}

#[derive(Debug)]
pub struct ClientBuilder {
    client: Option<reqwest::Client>,
    host: Option<url::Url>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            client: None,
            host: None,
        }
    }

    pub fn client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub fn host(mut self, host: impl IntoUrl) -> Self {
        self.host = Some(host.into_url().expect("invalid host"));
        self
    }

    pub fn build(self) -> Result<Client> {
        let inner = Inner {
            client: self.client.unwrap_or_else(reqwest::Client::new),
            host: self.host.with_context(|| "host is required")?,
        };

        Ok(Client {
            inner: Arc::new(inner),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    inner: Arc<Inner>,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub fn client(&self) -> &reqwest::Client {
        &self.inner.client
    }

    pub fn host(&self) -> &url::Url {
        &self.inner.host
    }

    pub async fn send<T: api::Request>(&self, request: &T) -> api::Result<T::Response> {
        request.send(&self.inner.client, &self.inner.host).await
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
}
