use std::{
    future::Future,
    sync::{Arc, Mutex},
    time::Duration,
};

use tokio::sync::mpsc;

use crate::{
    client::Client,
    feature::runtime::Runtime,
    job::{FinishedJob, Job},
    worker::worker::Worker,
};

pub trait Executor: Send + Sync + 'static {
    fn execute(
        &self,
        client: Client,
        job: Job,
    ) -> impl Future<Output = anyhow::Result<FinishedJob>> + Send;
}

#[derive(Debug, Clone)]
pub struct Config {
    pub max_contract: usize,
    pub max_job: usize,
    pub interval: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_contract: 8,
            max_job: 8,
            interval: Duration::from_secs(5),
        }
    }
}

pub struct Builder<T: Executor> {
    client: Option<Client>,
    runtimes: Option<Vec<Runtime>>,
    config: Option<Config>,
    executor: Option<T>,
}

impl<T: Executor> Default for Builder<T> {
    fn default() -> Self {
        Self {
            client: None,
            runtimes: None,
            config: None,
            executor: None,
        }
    }
}

impl<T: Executor> Builder<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn client(mut self, client: Client) -> Self {
        self.client = Some(client);
        self
    }

    pub fn runtimes(mut self, runtimes: &[Runtime]) -> Self {
        self.runtimes = Some(runtimes.to_vec());
        self
    }

    pub fn config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    pub fn executor(mut self, executor: T) -> Self {
        self.executor = Some(executor);
        self
    }

    pub fn build(self) -> BasicWorker<T> {
        let executor = Arc::new(self.executor.unwrap());

        BasicWorker {
            client: self.client.unwrap(),
            executor,
            runtimes: self.runtimes.unwrap(),
            config: self.config.unwrap(),
        }
    }
}

pub struct BasicWorker<T: Executor> {
    client: Client,
    executor: Arc<T>,
    runtimes: Vec<Runtime>,
    config: Config,
}

impl<T: Executor> BasicWorker<T> {
    pub fn builder() -> Builder<T> {
        Builder::new()
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let (tx, rx) = mpsc::channel::<Job>(self.config.max_contract);
        let worker = self.client.worker().new(&self.runtimes).await?;

        let executor = Self::launch_executor(self.client, self.executor, rx).await;
        let contractors = Self::launch_contractors(worker, tx, self.config).await;

        for contractor in contractors {
            contractor.await?;
        }

        Ok(())
    }

    async fn launch_executor(client: Client, executor: Arc<T>, mut rx: mpsc::Receiver<Job>) {
        tokio::spawn(async move {
            while let Some(job) = rx.recv().await {
                let executor = executor.clone();
                let client = client.clone();

                tokio::spawn(async move {
                    executor.execute(client, job).await.unwrap();
                    println!("job finished");
                });
            }
        });
    }

    async fn launch_contractors(
        worker: Worker,
        tx: mpsc::Sender<Job>,
        config: Config,
    ) -> Vec<tokio::task::JoinHandle<()>> {
        let contractors = (0..config.max_contract)
            .map(|_| {
                let tx = tx.clone();
                let contractor = worker.contractor();
                let timeout = config.interval;

                tokio::spawn(async move {
                    println!("contractor started");
                    while true {
                        if let Some(job) = contractor.contract(timeout, None).await.unwrap() {
                            println!("contractor got a job");
                            tx.send(job).await.unwrap();
                        }
                    }
                })
            })
            .collect::<Vec<_>>();

        contractors
    }
}
