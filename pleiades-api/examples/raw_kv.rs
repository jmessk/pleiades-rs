use pleiades_api::{api, Client};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::default();

    let namespace = {
        let request = api::kv::namespace::create::Request::builder()
            .consistency("none")
            .build();
        client.call_api(&request).await?
    };

    let _set = {
        let request = api::kv::value::set::Request::builder()
            .namespace_id(&namespace.namespace_id)
            .key("example_key")
            .value("example_value")
            .get()
            .append()
            .build();

        client.call_api(&request).await?
    };

    let get = {
        let request = api::kv::value::get::Request::builder()
            .namespace_id(namespace.namespace_id)
            .key("example_key")
            .build();

        client.call_api(&request).await?
    };

    println!("get: {}", get.value);

    Ok(())
}
