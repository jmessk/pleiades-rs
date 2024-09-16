use core::time;
use std::{
    future::Future,
    sync::{Arc, Mutex},
};

use tokio::sync::mpsc;

use crate::{
    api,
    client::Client,
    feature::{id::Id, runtime::Runtime},
    job::Job,
    worker::Worker,
    Contractor,
};

pub trait Executor {
    fn execute(self, job: Job) -> impl Future<Output = anyhow::Result<()>> + Send;
}

pub struct Config {
    pub max_contract: usize,
    pub max_job: usize,
    pub interval: std::time::Duration,
}

pub struct Builder<T: Executor> {
    client: Option<Client>,
    runtimes: Option<&'static [Runtime]>,
    config: Option<Config>,
    executor: Option<T>,
}

impl<T: Executor> Builder<T> {
    pub fn new() -> Self {
        Self {
            client: None,
            runtimes: None,
            config: None,
            executor: None,
        }
    }
}

pub struct BasicWorker<T: Executor> {
    client: Client,
    executor: Arc<T>,
    runtimes: &'static [Runtime],
    config: Config,
}

impl<T: Executor> BasicWorker<T> {
    pub fn builder() -> Builder<T> {
        Builder::new()
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let (tx, mut rx) = mpsc::channel::<Job>(self.config.max_contract);
        let worker = self.client.worker().new(self.runtimes).await?;

        todo!()
    }

    async fn launch_executor(&self, rx: mpsc::Receiver<Job>) -> anyhow::Result<()> {
        while let Some(job) = rx.recv().await {
            let executor = self.executor.clone();
            tokio::spawn(async move {
                executor.execute(job).await.unwrap();
            });
        }

        Ok(())
    }

    async fn launch_contractors(self, worker: Worker, tx: mpsc::Sender<Job>) -> anyhow::Result<()> {
        let _ = (0..self.config.max_contract)
            .map(|_| {
                let tx = tx.clone();
                let contractor = worker.contractor();
                let timeout = self.config.interval;

                tokio::spawn(async move {
                    while let Some(job) = contractor.contract(timeout, None).await.unwrap() {
                        tx.send(job).await.unwrap();
                    }
                });
            })
            .collect::<Vec<_>>();

        Ok(())
    }
}
