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
        let start = std::time::Instant::now();
        let code = client.new_blob("example lambda").await?;
        println!("1: {:?}", start.elapsed());

        // create lambda from code blob
        let start = std::time::Instant::now();
        let a = code.into_lambda("mecrm-rs+example").await?;
        println!("2: {:?}", start.elapsed());
        a
    };
    // or
    // let lambda = client.lambda_from_id("<lambda_id>").await?;

    // create input
    let input = client.new_blob("example input").await?;
    // or
    // let input = client.blob_from_id("<data_id>");

    // let job = lambda
    //     // run job
    //     .invoke(input)
    //     .await?
    //     // wait job finished
    //     .wait_finished(10)
    //     .await?;
    let start = std::time::Instant::now();
    let job = lambda.invoke(input).await?;
    println!("3: {:?}", start.elapsed());

    let job = job.wait_finished(10).await?;

    // get output
    let output = job.output.fetch().await?;
    println!("{:?}", output);

    Ok(())
}
