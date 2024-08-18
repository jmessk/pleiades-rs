use pleiades::api;
use pleiades::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // create arc client
    let client = Client::builder()
        // .host("https://pleiades.dolylab.cc/api/v0.5-snapshot/")
        // .host("http://192.168.168.127:8332/api/v0.5/")
        // .host("http://172.21.39.32:8332/api/v0.5/")
        // .host("http://pleiades.local:8332/api/v0.5/")
        .host("http://master.local/api/v0.5/")
        .build();

    let namespace = {
        let request = api::kv::namespace::Create::builder()
            .consistency("none")
            .build();
        client.send(&request).await?
    };

    let _set = {
        let request = api::kv::value::Set::builder()
            .namespace_id(&namespace.namespace_id)
            .key("example_key")
            .value("example_value")
            .get()
            .append()
            .build();

        client.send(&request).await?
    };

    let get = {
        let request = api::kv::value::Get::builder()
            .namespace_id(namespace.namespace_id)
            .key("example_key")
            .build();

        client.send(&request).await?
    };

    println!("get: {}", get.value);

    Ok(())
}
