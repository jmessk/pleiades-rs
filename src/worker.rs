use crate::{api, client::Client, job::Job, Id, Runtime};

pub struct Worker {
    pub(crate) client: Client,
    pub id: Id,
    pub runtimes: Vec<Runtime>,
}

impl Worker {
    pub async fn contractor(&self) -> Contractor {
        let request = api::WorkerContractRequest::builder()
            .worker_id(self.id.as_str())
            .timeout(10)
            .build();

        Contractor {
            client: self.client.clone(),
            request,
        }
    }
}

pub struct Contractor<'a> {
    pub(crate) client: Client,
    pub(crate) request: api::WorkerContractRequest<'a>,
}

impl<'a> Contractor<'a> {
    pub async fn contract(&self) -> anyhow::Result<Option<Job>> {
        let response = self.client.send(&self.request).await?;

        let job_id = match response.job_id {
            Some(job_id) => job_id,
            None => return Ok(None), // no job
        };

        let info = {
            let request = api::JobInfoRequest::builder().job_id(&job_id).build();
            self.client.send(&request).await?
        };

        

        todo!()
    }
}
