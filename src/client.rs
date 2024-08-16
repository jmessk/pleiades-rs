use std::sync::Arc;

use crate::{api, blob, job, lambda, worker};

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
