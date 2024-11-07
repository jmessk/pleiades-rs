use bytes::Bytes;

use crate::{
    api,
    client::Client,
    feature::{id::Id, runtime::Runtime},
    lambda::Lambda,
};

pub struct Selector {
    pub(crate) client: Client,
}

impl Selector {
    #[allow(clippy::new_ret_no_self, clippy::wrong_self_convention)]
    pub async fn new(self, data: impl Into<Bytes>) -> anyhow::Result<Blob> {
        let request = api::data::upload::Request { data: data.into() };
        let response = self.client.call_api(&request).await?;

        Ok(Blob {
            client: self.client,
            id: response.data_id.into(),
        })
    }

    #[allow(clippy::wrong_self_convention)]
    pub async fn from_id(self, id: impl Into<Id>) -> anyhow::Result<Blob> {
        Ok(Blob {
            client: self.client,
            id: id.into(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Blob {
    pub(crate) client: Client,
    pub id: Id,
}

impl Blob {
    pub async fn fetch(&self) -> anyhow::Result<Bytes> {
        let request = api::data::download::Request {
            data_id: self.id.as_str().into(),
        };

        let response = self.client.call_api(&request).await?;

        Ok(response.data)
    }

    pub async fn into_lambda(self, runtime: impl Into<Runtime>) -> anyhow::Result<Lambda> {
        let runtime: Runtime = runtime.into();

        let request = api::lambda::create::Request {
            runtime: runtime.as_str().into(),
            data_id: self.id.as_str().into(),
        };

        let response = self.client.call_api(&request).await?;

        Ok(crate::Lambda {
            client: self.client.clone(),
            id: response.lambda_id.into(),
            runtime,
            code: self,
        })
    }
}
