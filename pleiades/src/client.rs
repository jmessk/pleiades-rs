use std::sync::Arc;

use crate::{blob, job, lambda, worker};

#[derive(Debug, Clone, Default)]
pub struct Client {
    pub inner: Arc<pleiades_api::Client>,
}

impl Client {
    // pub fn builder() -> ClientBuilder {
    //     ClientBuilder::new()
    // }

    // pub async fn ping(&self) -> anyhow::Result<()> {
    //     self.inner.ping().await.with_context(|| "failed to ping")
    // }

    pub fn blob(&self) -> blob::Selector {
        blob::Selector {
            client: self,
        }
    }

    pub fn lambda(&self) -> lambda::Selector {
        lambda::Selector {
            client: self,
        }
    }

    pub fn worker(&self) -> worker::Selector {
        worker::Selector {
            client: self,
        }
    }

    pub fn job(&self) -> job::Selector {
        job::Selector {
            client: self,
        }
    }
}

// #[derive(Default)]
// pub struct ClientBuilder {
//     client: Option<reqwest::Client>,
//     host: Option<url::Url>,
//     // version: Option<String>,
// }

// impl ClientBuilder {
//     pub fn new() -> Self {
//         Self {
//             client: None,
//             host: None,
//             // version: None,
//         }
//     }

//     pub fn client(&mut self, client: reqwest::Client) -> &mut Self {
//         self.client = Some(client);
//         self
//     }

//     pub fn host(&mut self, host: impl reqwest::IntoUrl) -> &mut Self {
//         self.host = Some(host.into_url().expect("invalid host"));
//         self
//     }

//     // pub fn version(&mut self, version: impl Into<String>) -> &mut Self {
//     //     self.version = Some(version.into());
//     //     self
//     // }

//     pub fn build(&self) -> Client {
//         let client = self.client.clone().unwrap_or_default();
//         // let version = self.version.clone().expect("version is required");
//         let host = self.host.clone().expect("host is required");

//         // let host = format!("{host}/api/v{version}")
//         //     .parse()
//         //     .expect("invalid host");

//         Client {
//             inner: Arc::new(ClientRef { client, host }),
//         }
//     }
// }
