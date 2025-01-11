use std::time::Duration;

use pleiades_api::api;

use crate::{
    blob::Blob, client::Client, feature::{id::Id, runtime::Runtime}, job::Job
};

pub struct Selector<'a> {
    pub(crate) client: &'a Client,
}

impl<'a> Selector<'a> {
    #[allow(clippy::new_ret_no_self, clippy::wrong_self_convention)]
    pub async fn new(self, runtimes: &[Runtime]) -> anyhow::Result<Worker> {
        let register = {
            let runtimes = runtimes.iter().map(|r| r.as_str()).collect::<Vec<_>>();
            let request = api::worker::register::Request {
                runtimes: &runtimes,
            };

            self.client.inner.call_api(&request).await?
        };

        Ok(Worker {
            client: self.client.clone(),
            id: register.worker_id.into(),
            runtimes: runtimes.to_vec(),
        })
    }

    // #[allow(clippy::wrong_self_convention)]
    // pub async fn from_id(self, id: impl Into<Id>) -> anyhow::Result<Worker> {
    //     todo!()
    // }
}

pub struct Worker {
    pub(crate) client: Client,
    pub id: Id,
    pub runtimes: Vec<Runtime>,
}

impl Worker {
    pub fn contractor(&self) -> Contractor {
        Contractor {
            client: self.client.clone(),
            worker_id: self.id.clone(),
        }
    }
}

pub struct Contractor {
    pub(crate) client: Client,
    pub(crate) worker_id: Id,
}

impl Contractor {
    pub async fn contract(
        &self,
        timeout: Duration,
        tags: Option<&[&str]>,
    ) -> anyhow::Result<Option<Job<impl Blob>>> {
        let contract = {
            let request = api::worker::contract::Request {
                worker_id: self.worker_id.as_str().into(),
                timeout: timeout.as_secs(),
                tags: tags.unwrap_or_default(),
            };

            self.client.inner.call_api(&request).await?
        };

        let job_id = match contract.job_id {
            Some(job_id) => job_id,
            None => return Ok(None), // no job and early return
        };

        let job = self.client.job().from_id(job_id).await?;

        Ok(Some(job))
    }
}
