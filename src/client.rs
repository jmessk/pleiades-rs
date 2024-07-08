use reqwest::IntoUrl;

use crate::api;

#[derive(Debug, typed_builder::TypedBuilder)]
pub struct Client {
    #[builder(default = reqwest::Client::new())]
    client: reqwest::Client,

    #[builder(setter(transform = |url: impl IntoUrl| url.into_url().expect("Invalid Host")))]
    host: url::Url,
}

impl Client {
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    pub fn host(&self) -> &url::Url {
        &self.host
    }

    pub async fn request<T: api::Request>(&self, request: T) -> api::Result<T::Response> {
        request.send(self.client(), self.host()).await
    }
}
