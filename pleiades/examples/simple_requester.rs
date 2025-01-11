use pleiades::{Client, Lambda};
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::default();
    let lambda = create_lambda(&client).await?;

    // job input
    let input = client.blob().create(r#"{"a":3,"b":5}"#).await?;

    // create job
    let job = lambda.invoke(input, None).await?;

    // wait job finished
    let job = job.wait_finished(Duration::from_secs(10)).await?;

    // get output
    let output = job.output.fetch().await?;
    println!("job finished. output: {output:?}");

    Ok(())
}

async fn create_lambda(client: &Client) -> anyhow::Result<Lambda> {
    let code = r"
function add(a, b) {
    return a + b; 
}
";

    // create lambda code
    let code = client.blob().create(code).await?;

    // create lambda with runtime
    let lambda = code.into_lambda("pleiades+example").await?;

    Ok(lambda)
}
