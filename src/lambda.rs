use anyhow::Result;

use super::{blob::Blob, runtime::Runtime};
use super::{MecrmObject, ObjectBuilder};

pub struct Lambda {
    id: Option<String>,
    runtime: Option<Runtime>,
    blob: Option<Blob>,
}

impl MecrmObject for Lambda {
    fn new() -> impl ObjectBuilder {
        LambdaBuilder::new()
    }
}

impl Lambda {
    pub fn id(&self) -> &str {
        self.id.as_deref().unwrap()
    }

    pub fn runtime(&self) -> &Runtime {
        self.runtime.as_ref().unwrap()
    }

    pub fn blob(&self) -> &Blob {
        self.blob.as_ref().unwrap()
    }
}

struct LambdaBuilder {
    inner: Lambda,
}

impl ObjectBuilder for LambdaBuilder {
    fn new() -> LambdaBuilder {
        LambdaBuilder {
            inner: Lambda {
                id: None,
                runtime: None,
                blob: None,
            },
        }
    }

    fn build(self) -> Result<Lambda> {
        Ok(self.inner)
    }
}

impl LambdaBuilder {
    pub fn id(mut self, id: impl Into<String>) -> LambdaBuilder {
        self.inner.id = Some(id.into());
        self
    }

    pub fn runtime(mut self, runtime: Runtime) -> LambdaBuilder {
        self.inner.runtime = Some(runtime);
        self
    }

    pub fn blob(mut self, blob: Blob) -> LambdaBuilder {
        self.inner.blob = Some(blob);
        self
    }
}
