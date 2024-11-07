#[tokio::main]
async fn main() {
    let client = pleiades_api::Client {
        client: reqwest::Client::new(),
        base_url: url::Url::parse("https://mecrm.dolylab.cc/api/v0.5/").unwrap(),
    };

    let ping = client.ping().await.unwrap();
    println!("{:?}", ping);
}
