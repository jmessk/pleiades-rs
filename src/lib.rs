mod api;
mod blob;
mod job;
mod lambda;
mod runtime;
mod worker;

use anyhow::{Result, Context};

pub use blob::Blob;
pub use job::Job;
pub use lambda::Lambda;
pub use runtime::Runtime;
pub use worker::Worker;

pub trait MecrmObject {}

pub trait ObjectBuilder {
    type Output: MecrmObject;

    fn new(client: Client) -> Self;
    // fn id(self, id: impl Into<String>) -> Self::Output;
}

pub struct Client {
    client: reqwest::Client,
    host: url::Url,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    pub fn host(&self) -> &url::Url {
        &self.host
    }
}

pub struct ClientBuilder {
    host: Option<url::Url>,
    client: Option<reqwest::Client>,
}

impl ClientBuilder {
    pub fn new() -> ClientBuilder {
        ClientBuilder {
            host: None,
            client: None,
        }
    }

    pub fn client(mut self, client: reqwest::Client) -> ClientBuilder {
        self.client = Some(client);
        self
    }

    pub fn host(mut self, host: impl Into<String>) -> ClientBuilder {
        self.host = Some(url::Url::parse(&host.into()).unwrap());
        self
    }

    pub fn build(self) -> Result<Client> {
        Ok(Client {
            host: self.host.with_context(|| "Host is required")?,
            client: self.client.unwrap_or_else(|| reqwest::Client::new()),
        })
    }
}
