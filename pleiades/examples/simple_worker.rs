use std::time::Duration;
use pleiades::{Client, Job};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::default();

    // create worker
    let worker = client.worker().new(&["pleiades+example".into()]).await?;

    // create contractor
    let contractor = worker.contractor();

    // wait for job
    println!("waiting for job. timeout: 10s");

    while let Some(job) = contractor.contract(Duration::from_secs(10), None).await? {
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
    println!("input: {input:?}");

    // create output
    let output = client.blob().create(r#"{"output":8}"#).await?;

    // finish job
    job.finish(output).await?;

    Ok(())
}
