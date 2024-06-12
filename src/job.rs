use anyhow::Result;

use super::{blob::Blob, lambda::Lambda};
use super::{Handler, MecrmObject, ObjectBuilder};

pub struct Job {
    id: String,
    lambda: Lambda,
    input: Blob,
    output: Option<Blob>,
}

impl MecrmObject for Job {}

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
    client: Handler,
    lambda: Option<Lambda>,
    input: Option<Blob>,
}

impl ObjectBuilder for JobBuilder {
    type Output = Job;

    fn new(client: Handler) -> JobBuilder {
        JobBuilder {
            client,
            lambda: None,
            input: None,
        }
    }
}

impl JobBuilder {
    pub fn lambda(mut self, lambda: Lambda) -> JobBuilder {
        self.lambda = Some(lambda);
        self
    }

    pub fn input(mut self, input: Blob) -> JobBuilder {
        self.input = Some(input);
        self
    }

    pub async fn run(self, status: String, timeout: u32) -> Result<RequesterJob> {
        Ok(RequesterJob {
            id: "1234567890".to_string(),
            lambda: self.lambda.unwrap(),
            input: self.input.unwrap(),
            output: None,
        })
    }
}

pub struct RequesterJob {
    id: String,
    lambda: Lambda,
    input: Blob,
    output: Option<Blob>,
}

impl RequesterJob {
    pub async fn wait(self) -> Result<RequesterJob> {
        unimplemented!()
    }

    pub fn output(&self) -> &Blob {
        self.output.as_ref().unwrap()
    }
}
