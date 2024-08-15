use crate::{api, blob::Blob, Client, Id, Lambda};

pub enum Status {
    PreAssigned,
    Enqueued,
    Running,
    Finished(FinishedJob),
    Cancelled,
    Unknown(String),
}

pub struct Job {
    pub(crate) client: Client,
    pub id: Id,
    pub lambda: Lambda,
    pub input: Blob,
}

impl Job {
    const DEFAULT_TIMEOUT: u32 = 20;

    fn convert(&self, response: api::JobInfoResponse) -> Status {
        match response.job_status.as_str() {
            "PreAssigned" => Status::PreAssigned,
            "Running" => Status::Running,
            "Enqueued" => Status::Enqueued,
            "Finished" => {
                let output_id = response.output.unwrap().data_id;

                let output = Blob {
                    client: self.client.clone(),
                    id: output_id.into(),
                };

                Status::Finished(FinishedJob {
                    id: self.id.clone(),
                    lambda: self.lambda.clone(),
                    input: self.input.clone(),
                    output,
                })
            }
            "Cancelled" => Status::Cancelled,
            _ => Status::Unknown(response.job_status),
        }
    }

    pub async fn status(&self) -> anyhow::Result<Status> {
        let request = api::JobInfoRequest::builder()
            .job_id(self.id.as_str())
            .build();

        let response = self.client.send(&request).await?;

        Ok(self.convert(response))
    }

    pub async fn wait_status(&self, status: Status, timeout: u32) -> anyhow::Result<Self> {
        todo!()
    }

    pub async fn wait_finished(&self, timeout: u32) -> anyhow::Result<FinishedJob> {
        let request = api::JobInfoRequest::builder()
            .job_id(self.id.as_str())
            .except("Finished")
            .timeout(timeout)
            .build();

        let response = self.client.send(&request).await?;

        match self.convert(response) {
            Status::Finished(job) => Ok(job),
            _ => anyhow::bail!("job is not finished"),
        }
    }

    pub async fn cancel(&self) -> anyhow::Result<()> {
        todo!()
    }
}

pub struct FinishedJob {
    pub id: Id,
    pub lambda: Lambda,
    pub input: Blob,
    pub output: Blob,
}
