use std::sync::Arc;

use anyhow::{Context, Result};
use reqwest::IntoUrl;

use crate::{
    api,
    blob::{Blob, BlobBuilder},
    lambda::LambdaBuilder,
};

// #[derive(Debug, typed_builder::TypedBuilder)]
// pub struct Inner {
//     #[builder(default = reqwest::Client::new())]
//     client: reqwest::Client,
//     #[builder(setter(transform = |url: impl IntoUrl| url.into_url().expect("invalid host")))]
//     host: url::Url,
// }

#[derive(Debug)]
struct Inner {
    client: reqwest::Client,
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

    pub fn blob(&self) -> BlobBuilder {
        BlobBuilder::new(self.clone())
    }

    pub fn lambda(&self) -> LambdaBuilder {
        LambdaBuilder::new(self.clone())
    }
}
