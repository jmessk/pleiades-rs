use anyhow::Result;
use bytes::Bytes;

use crate::{
    api,
    blob::{Blob, RemoteBlob},
    id::Id,
    runtime::Runtime,
    Client,
};

#[derive(Debug)]
pub struct LambdaBuilder {
    client: Client,
    runtime: Option<Runtime>,
    code: Option<Bytes>,
}

impl LambdaBuilder {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            runtime: None,
            code: None,
        }
    }

    pub fn runtime(mut self, runtime: Runtime) -> Self {
        self.runtime = Some(runtime);
        self
    }

    pub fn code(mut self, code: Bytes) -> Self {
        self.code = Some(code);
        self
    }

    pub async fn build(self) -> Result<Lambda> {
        let runtime = self.runtime.expect("runtime is required");
        let code = self.code.expect("code is required");

        let blob = self.client.blob().data(code).build().await?;

        let create_lambda = api::LambdaCreateRequest::builder()
            .runtime(runtime.full())
            .data_id(blob.id().as_str())
            .build();

        let response = self.client.send(create_lambda).await?;

        Ok(Lambda {
            id: response.lambda_id.into(),
            runtime,
            blob,
        })
    }
}

#[derive(Debug)]
pub struct Lambda {
    id: Id,
    runtime: Runtime,
    blob: Blob,
}

impl Lambda {
    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }

    pub fn blob(&self) -> &Blob {
        &self.blob
    }
}

#[derive(Debug)]
pub struct RemoteLambda {
    client: Client,
    id: Id,
    blob: RemoteBlob,
}

impl RemoteLambda {
    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }

    pub fn blob(&self) -> &Blob {
        &self.blob
    }
}
