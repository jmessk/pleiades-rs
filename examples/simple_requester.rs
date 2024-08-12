#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = pleiades::Client::builder()
        // .host("https://pleiades.dolylab.cc/api/v0.5-snapshot/")
        // .host("http://192.168.168.127:8332/api/v0.5/")
        // .host("http://172.21.39.32:8332/api/v0.5/")
        .host("http://pleiades.local:8332/api/v0.5/")
        .build()?;

    // create lambda
    let lambda = {
        // create lambda code as blob
        let code = client.new_blob("test lambda").await?;

        // define runtime
        let runtime = pleiades::Runtime::builder()
            .base("mecrm-rs")
            .add_feature("example")
            .build();

        // create lambda
        code.into_lambda(runtime).await?
    };

    // create input
    let input = client.new_blob("test input").await?;

    let job = lambda
        // run job
        .invoke(input)
        .await?
        // wait job finished
        .wait_finished(10)
        .await?;

    // get output
    let output = job.output.fetch().await?;
    println!("{:?}", output);

    Ok(())
}
