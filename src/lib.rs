mod api;
mod blob;
mod job;
mod lambda;
mod runtime;
mod worker;

use std::sync::Arc;

use anyhow::Result;

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
    host: url::Url,
    client: Arc<reqwest::Client>,
}

pub struct ClientBuilder {
    host: Option<url::Url>,
    client: Option<Arc<reqwest::Client>>,
}
