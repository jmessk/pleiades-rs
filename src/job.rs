use anyhow::Result;

use super::{blob::Blob, lambda::Lambda};
use super::{MecrmObject, ObjectBuilder};

pub struct Job {
    id: String,
    lambda: Lambda,
    input: Blob,
    output: Option<Blob>,
}

impl MecrmObject for Job {
    fn new() -> impl ObjectBuilder {
        JobBuilder::new()
    }
}

impl Job {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn lambda(&self) -> &Lambda {
        &self.lambda
    }

    pub fn input(&self) -> &Blob {
        &self.input
    }

    pub fn output(&self) -> &Blob {
        self.output.as_ref().unwrap()
    }
}

struct JobBuilder {
    id: Option<String>,
    lambda: Option<Lambda>,
    input: Option<Blob>,
    output: Option<Blob>,
}

impl ObjectBuilder for JobBuilder {
    fn new() -> JobBuilder {
        JobBuilder {
            id: None,
            lambda: None,
            input: None,
            output: None,
        }
    }

    fn build(self) -> Result<impl MecrmObject> {
        Ok(Job {
            id: self.id.unwrap(),
            lambda: self.lambda.unwrap(),
            input: self.input.unwrap(),
            output: self.output,
        })
    }
}

impl JobBuilder {
    pub fn id(mut self, id: impl Into<String>) -> JobBuilder {
        self.id = Some(id.into());
        self
    }

    pub fn lambda(mut self, lambda: Lambda) -> JobBuilder {
        self.lambda = Some(lambda);
        self
    }

    pub fn input(mut self, input: Blob) -> JobBuilder {
        self.input = Some(input);
        self
    }

    pub fn run(self) -> JobForRequester {
        unimplemented!()
    }
}

struct JobForRequester {
    inner: Job,
}

impl JobForRequester {
    pub fn is_done(&self) -> bool {
        unimplemented!()
    }

    pub fn wait(self) -> JobForWorker {
        unimplemented!()
    }

    pub fn output(self) -> Blob {
        unimplemented!()
    }
}

pub struct JobForWorker {
    inner: Job,
}

impl JobForWorker {
    pub fn id(&self) -> &str {
        self.inner.id()
    }

    pub fn lambda(&self) -> &Lambda {
        self.inner.lambda()
    }

    pub fn input(&self) -> &Blob {
        self.inner.input()
    }

    pub fn finish(self, output: Blob) -> Result<()> {
        unimplemented!()
    }
}
