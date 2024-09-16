use pleiades::{Client, Lambda};
use std::time::Duration;

fn init() -> Client {
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // create client
    let client = Client::builder()
        .host("http://master.local/api/v0.5/")
        .build();

    client
}

async fn create_lambda(client: &Client) -> anyhow::Result<Lambda> {
    let code = r"
        function add(a, b) {
            return a + b; 
        }";

    // create lambda code
    let code = client.blob().new(code).await?;

    // create lambda with runtime
    let lambda = code.into_lambda("mecrm-rs+example").await?;

    Ok(lambda)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = init();
    let lambda = create_lambda(&client).await?;

    // job input
    let input = client.blob().new(r#"{"a":3,"b":5}"#).await?;

    // create job
    let job = lambda.invoke(input, None).await?;

    // wait job finished
    let job = job.wait_finished(Duration::from_secs(10)).await?;

    // get output
    let output = job.output.fetch().await?;
    println!("job finished. output: {:?}", output);

    Ok(())
}
