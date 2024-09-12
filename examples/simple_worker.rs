use pleiades::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = init();

    // create worker
    let worker = client.worker().new(&["mecrm-rs+example".into()]).await?;

    // create contractor
    let contractor = worker.contractor();

    // wait for job
    println!("waiting for job");
    println!("timeout: 10s");

    while let Some(job) = contractor.contract(10, &[]).await? {
        println!("contracted job");
        let client = client.clone();

        tokio::spawn(async move {
            execute(&client, job).await.unwrap();
        });
    }

    println!("No more job");
    Ok(())
}

async fn execute(client: &pleiades::Client, job: pleiades::Job) -> anyhow::Result<()> {
    // get input
    let input = job.input.fetch().await?;

    // do something
    println!("input: {:?}", input);

    // create output
    let output = client.blob().new("example output").await?;

    // finish job
    job.finish(output).await?;
    println!("finished job");

    Ok(())
}

fn init() -> Client {
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::ERROR)
        .init();

    Client::builder()
        .host("http://pleiades.local/api/v0.5/")
        .build()
}
