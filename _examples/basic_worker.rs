use pleiades::{
    executor::{BasicWorker, Config, Executor},
    Client,
};

struct PingSender {
    client: reqwest::Client,
}

impl Executor for PingSender {
    async fn execute(
        &self,
        client: Client,
        job: pleiades::Job,
    ) -> anyhow::Result<pleiades::FinishedJob> {
        self.client
            .get("http://master.local/api/v0.5/")
            .send()
            .await?
            .text()
            .await
            .inspect(|text| {
                println!("Ping response: {}", text);
            });

        let output = client.blob().new("pinged").await?;

        Ok(job.finish(output).await?)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::builder()
        .host("http://master.local/api/v0.5/")
        .build();

    let executor = PingSender {
        client: reqwest::Client::new(),
    };

    let worker = BasicWorker::builder()
        .client(client)
        .executor(executor)
        .runtimes(&["pleiades+example".into()])
        .config(Config::default())
        .build();

    worker.run().await?;

    Ok(())
}
