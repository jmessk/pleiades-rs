use pleiades::Client;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::default();

    // create lambda
    let code = client.blob().new("input.a + input.b").await?;
    let lambda = code.into_lambda("mecrm-rs+example").await?;

    // create input
    let input = client.blob().new(r#"{"a":3,"b":5}"#).await?;

    // create job
    let job = lambda.invoke(input).await?;
    // wait job finished
    let job = job.wait_finished(Duration::from_secs(10)).await?;

    // get output
    let output = job.output.fetch().await?;
    println!("output: {:?}", output);

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
