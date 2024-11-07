#[tokio::main]
async fn main() {
    let client = pleiades_api::Client::default();

    let ping = client.ping().await.unwrap();
    println!("{:?}", ping);
}
