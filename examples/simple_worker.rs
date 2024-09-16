use std::time::Duration;

use pleiades::{Client, Job};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let client = Client::builder()
        .host("http://master.local/api/v0.5/")
        .build();

    // create worker
    let worker = client.worker().new(&["mecrm-rs+example".into()]).await?;

    // create contractor
    let contractor = worker.contractor();

    // wait for job
    println!("waiting for job. timeout: 10s");

    while let Some(job) = contractor.contract(Duration::from_secs(10), &[]).await? {
        let client = client.clone();

        tokio::spawn(async move {
            execute(&client, job).await.unwrap();
            println!("finished job");
        });
    }

    println!("no more job");
    Ok(())
}

async fn execute(client: &Client, job: Job) -> anyhow::Result<()> {
    // get input
    let _code = job.lambda.code.fetch().await?;
    let input = job.input.fetch().await?;

    // process input
    println!("input: {:?}", input);

    // create output
    let output = client.blob().new(r#"{ "output": 8 }"#).await?;

    // finish job
    job.finish(output).await?;

    Ok(())
}
