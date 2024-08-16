#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::ERROR)
        .init();

    let client = pleiades::Client::builder()
        // .host("https://pleiades.dolylab.cc/api/v0.5-snapshot/")
        // .host("http://192.168.168.127:8332/api/v0.5/")
        // .host("http://172.21.39.32:8332/api/v0.5/")
        // .host("http://pleiades.local:8332/api/v0.5/")
        .host("http://master.local/api/v0.5/")
        .build();

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
