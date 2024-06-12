mod api;
mod blob;
mod job;
mod lambda;
mod runtime;
// mod worker;

use anyhow::{Context, Result};

pub use blob::Blob;
pub use job::Job;
pub use lambda::Lambda;
use reqwest::IntoUrl;
pub use runtime::Runtime;
// pub use worker::Worker;

pub trait MecrmObject {}

pub trait ObjectBuilder {
    type Output: MecrmObject;

    fn new(client: Handler) -> Self;
    // fn id(self, id: impl Into<String>) -> Self::Output;
}

#[derive(Debug)]
pub struct Handler {
    client: reqwest::Client,
    host: url::Url,
}

impl Handler {
    pub fn builder() -> HandlerBuilder {
        HandlerBuilder::new()
    }

    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    pub fn host(&self) -> &url::Url {
        &self.host
    }
}

pub struct HandlerBuilder {
    host: Option<url::Url>,
    client: Option<reqwest::Client>,
}

impl HandlerBuilder {
    pub fn new() -> HandlerBuilder {
        HandlerBuilder {
            host: None,
            client: None,
        }
    }

    pub fn client(mut self, client: reqwest::Client) -> HandlerBuilder {
        self.client = Some(client);
        self
    }

    pub fn host(mut self, host: impl IntoUrl) -> HandlerBuilder {
        self.host = Some(host.into_url().expect("Invalid Host"));
        self
    }

    pub fn build(self) -> Result<Handler> {
        Ok(Handler {
            host: self.host.with_context(|| "Host is required")?,
            client: self.client.unwrap_or_else(|| reqwest::Client::new()),
        })
    }
}
