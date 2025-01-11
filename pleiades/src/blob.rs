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
    pub async fn create(self, data: impl Into<Bytes>) -> anyhow::Result<LocalBlob> {
        let data = data.into();

        let request = api::data::upload::Request { data: data.clone() };
        let response = self.client.inner.call_api(&request).await?;

        Ok(LocalBlob {
            client: self.client.clone(),
            id: response.data_id.into(),
            data,
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

pub(crate) trait Blob: Sized + Clone {
    fn id(&self) -> &Id;
}

#[derive(Debug, Clone)]
pub struct RemoteBlob {
    pub(crate) client: Client,
    pub id: Id,
}

impl Blob for RemoteBlob {
    fn id(&self) -> &Id {
        &self.id
    }
}

impl RemoteBlob {
    pub async fn fetch(&self) -> anyhow::Result<LocalBlob> {
        let request = api::data::download::Request {
            data_id: self.id.as_str().into(),
        };
        let response = self.client.inner.call_api(&request).await?;

        Ok(LocalBlob {
            client: self.client.clone(),
            id: self.id.clone(),
            data: response.data,
        })
    }

    pub async fn into_lambda(self, runtime: impl Into<Runtime>) -> anyhow::Result<Lambda<Self>> {
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

#[derive(Debug, Clone)]
pub struct LocalBlob {
    pub(crate) client: Client,
    pub id: Id,
    pub data: Bytes,
}

impl Blob for LocalBlob {
    fn id(&self) -> &Id {
        &self.id
    }
}

impl LocalBlob {
    pub async fn into_lambda(self, runtime: impl Into<Runtime>) -> anyhow::Result<Lambda<Self>> {
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
