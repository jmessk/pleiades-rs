use crate::api;

pub struct Client {
    client: reqwest::Client,
    base_url: url::Url,
}

impl Client {
    pub async fn call_api<T: api::CoreRequest>(&self, request: &T) -> api::Result<T::Response> {
        request.send(&self.client, &self.base_url).await
    }

    pub async fn ping(&self) {
        
    }
}
