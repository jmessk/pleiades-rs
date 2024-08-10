use pleiades::{Blob, Client, Runtime};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::builder()
        // .host("https://pleiades.dolylab.cc/api/v0.5-snapshot/")
        // .host("http://192.168.168.127:8332/api/v0.5/")
        // .host("http://172.21.39.32:8332/api/v0.5/")
        .host("http://pleiades.local:8332/api/v0.5/")
        .build()?;

    let code = Blob::from_id("1");
    let runtime = Runtime::builder()
        .base("example")
        .add_feature("mecrm-rs")
        .build();

    let lambda = client.create_lambda(code, runtime).await?;
    let input = client.upload("").await?;

    let job = lambda.invoke(input).await?;

    Ok(())
}
