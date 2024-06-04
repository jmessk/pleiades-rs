use anyhow::Result;

use super::{blob::Blob, lambda::Lambda};
use super::{MecrmObject, ObjectBuilder};

pub struct Job {
    id: Option<String>,
    lambda: Option<Lambda>,
    input: Option<Blob>,
    output: Option<Blob>,
}

impl MecrmObject for Job {
    fn new() -> impl ObjectBuilder {
        JobBuilder::new()
    }
}

impl Job {
    pub fn id(&self) -> &str {
        self.id.as_deref().unwrap()
    }

    pub fn lambda(&self) -> &Lambda {
        self.lambda.as_ref().unwrap()
    }

    pub fn input(&self) -> &Blob {
        self.input.as_ref().unwrap()
    }

    pub fn output(&self) -> &Blob {
        self.output.as_ref().unwrap()
    }
}

struct JobBuilder {
    inner: Job,
}

impl ObjectBuilder for JobBuilder {
    fn new() -> JobBuilder {
        JobBuilder {
            inner: Job {
                id: None,
                lambda: None,
                input: None,
                output: None,
            },
        }
    }

    fn build(self) -> Result<impl MecrmObject> {
        Ok(Job {
            id: Some(self.inner.id.unwrap()),
            lambda: Some(self.inner.lambda.unwrap()),
            input: Some(self.inner.input.unwrap()),
            output: Some(self.inner.output.unwrap()),
        })
    }
}

impl JobBuilder {
    pub fn id(mut self, id: impl Into<String>) -> JobBuilder {
        self.inner.id = Some(id.into());
        self
    }

    pub fn lambda(mut self, lambda: Lambda) -> JobBuilder {
        self.inner.lambda = Some(lambda);
        self
    }

    pub fn input(mut self, input: Blob) -> JobBuilder {
        self.inner.input = Some(input);
        self
    }

    pub fn run(self) -> RequesterJob {
        RequesterJob {
            inner: self.inner,
        }
    }
}

struct RequesterJob {
    inner: Job,
}

impl RequesterJob {
    pub fn is_done(&self) -> bool {
        unimplemented!()
    }

    pub fn wait(self) -> WorkerJob {
        WorkerJob { inner: self.inner }
    }

    pub fn output(self) -> Blob {
        self.inner.output.unwrap()
    }
}

pub struct WorkerJob {
    inner: Job,
}

impl WorkerJob {
    pub fn id(&self) -> &str {
        self.inner.id.as_deref().unwrap()
    }

    pub fn lambda(&self) -> &Lambda {
        self.inner.lambda.as_ref().unwrap()
    }

    pub fn input(&self) -> &Blob {
        self.inner.input.as_ref().unwrap()
    }

    pub fn finish(self, output: Blob) -> Job {
        Job {
            id: self.inner.id,
            lambda: self.inner.lambda,
            input: self.inner.input,
            output: Some(output),
        }
    }
}
