use crate::{blob::Blob, runtime::Runtime};

pub struct Lambda {
    id: String,
    runtime: Runtime,
    blob: Blob,
}

impl Lambda {
    pub fn new() -> LambdaBuilder {
        LambdaBuilder::new()
    }

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
    id: Option<String>,
    runtime: Option<Runtime>,
    blob: Option<Blob>,
}

impl LambdaBuilder {
    pub fn new() -> LambdaBuilder {
        LambdaBuilder {
            id: None,
            runtime: None,
            blob: None,
        }
    }

    pub fn build(self) -> Lambda {
        Lambda {
            id: self.id.expect("id is required"),
            runtime: self.runtime.expect("runtime is required"),
            blob: self.blob.expect("blob is required"),
        }
    }

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
