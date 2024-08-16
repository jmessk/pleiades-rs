use crate::{api, blob::Blob, Client, Id, Lambda};

pub struct Selector {
    pub(crate) client: Client,
}

impl Selector {
    #[allow(clippy::wrong_self_convention)]
    pub async fn from_id(self, id: impl Into<Id>) -> anyhow::Result<Job> {
        let job_id: Id = id.into();

        let info = {
            let request = api::JobInfoRequest::builder()
                .job_id(job_id.as_str())
                .build();

            self.client.send(&request).await?
        };

        let lambda = Lambda {
            client: self.client.clone(),
            id: info.lambda.lambda_id.into(),
            runtime: info.lambda.runtime.into(),
            blob: self.client.blob().from_id(info.lambda.data_id),
        };

        let input = self.client.blob().from_id(info.input.data_id);

        Ok(Job {
            client: self.client,
            id: job_id,
            lambda,
            input,
        })
    }
}

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
    // const DEFAULT_TIMEOUT: u32 = 20;

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

    // pub async fn wait_status(&self, status: Status, timeout: u32) -> anyhow::Result<Self> {
    //     todo!()
    // }

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

    pub async fn finish(self, output: Blob) -> anyhow::Result<FinishedJob> {
        let _update = {
            let request = api::JobUpdateRequest::builder()
                .job_id(self.id.as_str())
                .status("finished")
                .data_id(output.id.as_str())
                .build();

            self.client.send(&request).await?
        };

        Ok(FinishedJob {
            id: self.id,
            lambda: self.lambda,
            input: self.input,
            output,
        })
    }
}

pub struct FinishedJob {
    pub id: Id,
    pub lambda: Lambda,
    pub input: Blob,
    pub output: Blob,
}
