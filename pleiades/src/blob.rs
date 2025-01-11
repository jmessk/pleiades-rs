use bytes::Bytes;

use pleiades_api::api;

use crate::{
    client::Client,
    feature::{id::Id, runtime::Runtime},
    lambda::Lambda,
};

pub struct Selector<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> Selector<'a> {
    pub async fn create(self, data: impl Into<Bytes>) -> anyhow::Result<RemoteBlob> {
        let request = api::data::upload::Request { data: data.into() };
        let response = self.client.inner.call_api(&request).await?;

        Ok(RemoteBlob {
            client: self.client.clone(),
            id: response.data_id.into(),
        })
    }

    #[allow(clippy::wrong_self_convention)]
    pub async fn from_id(self, id: impl Into<Id>) -> anyhow::Result<RemoteBlob> {
        Ok(RemoteBlob {
            client: self.client.clone(),
            id: id.into(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct RemoteBlob {
    pub(crate) client: Client,
    pub id: Id,
}

impl RemoteBlob {
    pub async fn fetch(&self) -> anyhow::Result<Bytes> {
        let request = api::data::download::Request {
            data_id: self.id.as_str().into(),
        };
        let response = self.client.inner.call_api(&request).await?;

        Ok(response.data)
    }

    pub async fn into_lambda(self, runtime: impl Into<Runtime>) -> anyhow::Result<Lambda> {
        let runtime: Runtime = runtime.into();

        let request = api::lambda::create::Request {
            runtime: runtime.as_str().into(),
            data_id: self.id.as_str().into(),
        };
        let response = self.client.inner.call_api(&request).await?;

        Ok(crate::Lambda {
            client: self.client.clone(),
            id: response.lambda_id.into(),
            runtime,
            code: self,
        })
    }
}

pub struct LocalBlob {
    pub(crate) client: Client,
    pub id: Id,
    pub data: Bytes,
}


