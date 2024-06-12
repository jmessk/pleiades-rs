use anyhow::{bail, Result};

use super::{blob::Blob, runtime::Runtime};
use super::{Client, MecrmObject, ObjectBuilder};

pub struct Lambda {
    id: String,
    runtime: Runtime,
    blob: Blob,
}

impl MecrmObject for Lambda {}

impl Lambda {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }

    pub fn blob(&self) -> &Blob {
        &self.blob
    }
}

pub struct LambdaBuilder {
    client: Client,
    id: Option<String>,
    runtime: Option<Runtime>,
    blob: Option<Blob>,
}

impl ObjectBuilder for LambdaBuilder {
    type Output = Lambda;

    fn new(client: Client) -> LambdaBuilder {
        LambdaBuilder {
            client,
            id: None,
            runtime: None,
            blob: None,
        }
    }

    // fn id(self, id: impl Into<String>) -> Lambda {
    //     Lambda {
    //         id: id.into(),
    //         runtime: self.runtime.unwrap(),
    //         blob: self.blob.unwrap(),
    //     }
    // }
}

impl LambdaBuilder {
    pub fn runtime(mut self, runtime: Runtime) -> LambdaBuilder {
        self.runtime = Some(runtime);
        self
    }

    pub fn blob(mut self, blob: Blob) -> LambdaBuilder {
        self.blob = Some(blob);
        self
    }

    pub async fn post(self) -> Result<Lambda> {
        if self.runtime.is_none() {
            bail!("Lambda runtime is not set");
        }

        if self.blob.is_none() {
            bail!("Lambda blob is not set");
        }

        Ok(Lambda {
            id: "1234567890".to_string(),
            runtime: self.runtime.unwrap(),
            blob: self.blob.unwrap(),
        })
    }
}
