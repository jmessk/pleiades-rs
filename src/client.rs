use std::sync::Arc;

use crate::{api, blob, job, lambda, worker};

// #[derive(Debug, Clone, typed_builder::TypedBuilder)]
// pub struct Client {
//     #[builder(default = reqwest::Client::new())]
//     client: reqwest::Client,

//     #[builder(setter(transform = |url: impl reqwest::IntoUrl| Arc::new(url.into_url().expect("invalid host"))))]
//     host: Arc<url::Url>,
// }

#[derive(Debug)]
struct ClientRef {
    client: reqwest::Client,
    host: url::Url,
}

#[derive(Debug, Clone)]
pub struct Client {
    inner: Arc<ClientRef>,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub async fn call_api<T: api::Request>(&self, request: &T) -> api::Result<T::Response> {
        request.send(&self.inner.client, &self.inner.host).await
    }

    pub fn blob(&self) -> blob::Selector {
        blob::Selector {
            client: self.clone(),
        }
    }

    pub fn lambda(&self) -> lambda::Selector {
        lambda::Selector {
            client: self.clone(),
        }
    }

    pub fn worker(&self) -> worker::Selector {
        worker::Selector {
            client: self.clone(),
        }
    }

    pub fn job(&self) -> job::Selector {
        job::Selector {
            client: self.clone(),
        }
    }
}

#[derive(Default)]
pub struct ClientBuilder {
    client: Option<reqwest::Client>,
    host: Option<url::Url>,
    // version: Option<String>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            client: None,
            host: None,
            // version: None,
        }
    }

    pub fn client(&mut self, client: reqwest::Client) -> &mut Self {
        self.client = Some(client);
        self
    }

    pub fn host(&mut self, host: impl reqwest::IntoUrl) -> &mut Self {
        self.host = Some(host.into_url().expect("invalid host"));
        self
    }

    // pub fn version(&mut self, version: impl Into<String>) -> &mut Self {
    //     self.version = Some(version.into());
    //     self
    // }

    pub fn build(&self) -> Client {
        let client = self.client.clone().unwrap_or_default();
        // let version = self.version.clone().expect("version is required");
        let host = self.host.clone().expect("host is required");

        // let host = format!("{host}/api/v{version}")
        //     .parse()
        //     .expect("invalid host");

        Client {
            inner: Arc::new(ClientRef { client, host }),
        }
    }
}
