use anyhow::Result;

use super::runtime::Runtime;
use super::{MecrmObject, ObjectBuilder};

pub struct Worker {
    id: String,
    runtimes: Vec<Runtime>,
}

impl MecrmObject for Worker {
    type Builder = WorkerBuilder;

    fn new() -> WorkerBuilder {
        WorkerBuilder::new()
    }
}

impl Worker {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn runtimes(&self) -> &Vec<Runtime> {
        &self.runtimes
    }
}

struct WorkerBuilder {
    id: Option<String>,
    runtimes: Option<Vec<Runtime>>,
}

impl ObjectBuilder for WorkerBuilder {
    type Output = Worker;

    fn new() -> WorkerBuilder {
        WorkerBuilder {
            id: None,
            runtimes: None,
        }
    }

    fn build(self) -> Result<Worker> {
        Ok(Worker {
            id: self.id.unwrap(),
            runtimes: self.runtimes.unwrap(),
        })
    }
}

impl WorkerBuilder {
    pub fn id(mut self, id: impl Into<String>) -> WorkerBuilder {
        self.id = Some(id.into());
        self
    }

    pub fn runtimes(mut self, runtimes: impl Into<Vec<Runtime>>) -> WorkerBuilder {
        self.runtimes = Some(runtimes.into());
        self
    }
}
