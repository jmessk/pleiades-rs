use pleiades::api;
use pleiades::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::ERROR)
        .init();

    let client = Client::builder()
        .host("http://pleiades.local/api/v0.5/")
        .build();

    let namespace = {
        let request = api::kv::namespace::Create::builder()
            .consistency("none")
            .build();
        client.call_api(&request).await?
    };

    let _set = {
        let request = api::kv::value::Set::builder()
            .namespace_id(&namespace.namespace_id)
            .key("example_key")
            .value("example_value")
            .get()
            .append()
            .build();

        client.call_api(&request).await?
    };

    let get = {
        let request = api::kv::value::Get::builder()
            .namespace_id(namespace.namespace_id)
            .key("example_key")
            .build();

        client.call_api(&request).await?
    };

    println!("get: {}", get.value);

    Ok(())
}
