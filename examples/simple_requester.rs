use pleiades::{Client, Lambda};
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let client = Client::builder()
        .host("http://master.local/api/v0.5/")
        .build();

    // process using MEC-RM
    let lambda = create_lambda(&client).await?;
    let output = execute(&client, &lambda).await?;

    println!("finished. output: {:?}", output);

    Ok(())
}

async fn create_lambda(client: &Client) -> anyhow::Result<Lambda> {
    let code = "function add(a, b) { return a + b; }";

    // create lambda code
    let code = client.blob().new(code).await?;

    // create lambda with runtime
    let lambda = code.into_lambda("mecrm-rs+example").await?;

    Ok(lambda)
}

async fn execute(client: &Client, lambda: &Lambda) -> anyhow::Result<bytes::Bytes> {
    // create input
    let input = client.blob().new(r#"{"a":3,"b":5}"#).await?;

    // create job
    let job = lambda.invoke(input, &[]).await?;

    // wait job finished
    let job = job.wait_finished(Duration::from_secs(10)).await?;

    // get output
    let output = job.output.fetch().await?;

    Ok(output)
}
