#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = pleiades::Client::builder()
        // .host("https://pleiades.dolylab.cc/api/v0.5-snapshot/")
        // .host("http://192.168.168.127:8332/api/v0.5/")
        // .host("http://172.21.39.32:8332/api/v0.5/")
        // .host("http://pleiades.local:8332/api/v0.5/")
        .host("http://master.local/api/v0.5/")
        .build();

    // create lambda
    let lambda = {
        // create lambda code as blob
        let code = client.blob().new("example lambda").await?;

        // create lambda from code blob
        code.into_lambda("mecrm-rs+example").await?
    };

    // create input
    let input = client.blob().new("example input").await?;

    let job = lambda
        // run job
        .invoke(input)
        .await?
        // wait job finished
        .wait_finished(10)
        .await?;

    // get output
    let output = job.output.fetch().await?;
    println!("output: {:?}", output);

    Ok(())
}
