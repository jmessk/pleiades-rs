use std::sync::Arc;

use anyhow::{Context, Result};
use bytes::Bytes;
use reqwest::IntoUrl;

use crate::{api, job::Job, Blob, Id, Lambda, Runtime};

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

    pub async fn send<T: api::Request>(&self, request: T) -> api::Result<T::Response> {
        request.send(self.client(), self.host()).await
    }

    pub async fn upload(&self, data: impl Into<Bytes>) -> Result<Blob> {
        let request = api::DataUploadRequest::builder()
            .data(data.into().clone())
            .build();
        let response = self.send(request).await?;

        Ok(Blob {
            id: response.data_id.into(),
        })
    }

    pub async fn download(&self, id: impl Into<Id>) -> Result<Bytes> {
        let id: Id = id.into();
        let request = api::DataDownloadRequest::builder()
            .data_id(id.as_str())
            .build();

        let response = self.send(request).await?;

        Ok(response.data)
    }

    pub async fn create_lambda(&self, code: Blob, runtime: Runtime) -> Result<Lambda> {
        let request = api::LambdaCreateRequest::builder()
            .runtime("")
            .data_id(code.id.as_str())
            .build();

        let response = self.send(request).await?;

        Ok(Lambda {
            client: self.clone(),
            id: response.lambda_id.into(),
            runtime,
            blob: code,
        })
    }
}
