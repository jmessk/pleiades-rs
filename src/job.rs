use crate::{blob::Blob, Client, Id, Lambda};

pub enum Status {
    PreAssigned,
    Running,
    Finished(FinishedJob),
    Cancelled,
}

pub struct Job {
    pub client: Client,
    pub id: Id,
    pub lambda: Lambda,
    pub input: Blob,
}

impl Job {
    pub async fn status(&self) -> anyhow::Result<Status> {
        todo!()
    }

    pub async fn wait(&self, status: Status, timeout: u32) -> anyhow::Result<Self> {
        todo!()
    }

    pub async fn wait_finished(&self, timeout: u32) -> anyhow::Result<FinishedJob> {
        todo!()
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
