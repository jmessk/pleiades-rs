use pleiades::Client;

#[tokio::main]
async fn main() {
    let client = Client::builder()
        // .host("https://pleiades.dolylab.cc/api/v0.5-snapshot/")
        // .host("http://192.168.168.127:8332/api/v0.5/")
        // .host("http://172.21.39.32:8332/api/v0.5/")
        .host("http://pleiades.local:8332/api/v0.5/")
        .build()
        .unwrap();

    // let input = client.blob().data(bytes::Bytes::from_static(b"")).build();
}
