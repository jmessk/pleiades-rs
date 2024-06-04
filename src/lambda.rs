use anyhow::Result;

use super::{blob::Blob, runtime::Runtime};
use super::{MecrmObject, ObjectBuilder};

pub struct Lambda {
    id: String,
    runtime: Runtime,
    blob: Blob,
}

impl MecrmObject for Lambda {
    fn new() -> impl ObjectBuilder {
        LambdaBuilder::new()
    }
}

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

struct LambdaBuilder {
    id: Option<String>,
    runtime: Option<Runtime>,
    blob: Option<Blob>,
}

impl ObjectBuilder for LambdaBuilder {
    fn new() -> LambdaBuilder {
        LambdaBuilder {
            id: None,
            runtime: None,
            blob: None,
        }
    }

    fn build(self) -> Result<Lambda> {
        Ok(Lambda {
            id: self.id.unwrap(),
            runtime: self.runtime.unwrap(),
            blob: self.blob.unwrap(),
        })
    }
}

impl LambdaBuilder {
    pub fn id(mut self, id: impl Into<String>) -> LambdaBuilder {
        self.id = Some(id.into());
        self
    }

    pub fn runtime(mut self, runtime: Runtime) -> LambdaBuilder {
        self.runtime = Some(runtime);
        self
    }

    pub fn blob(mut self, blob: Blob) -> LambdaBuilder {
        self.blob = Some(blob);
        self
    }
}
