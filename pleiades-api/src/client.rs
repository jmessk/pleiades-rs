use crate::api;

#[derive(Debug)]
pub struct Client {
    pub client: reqwest::Client,
    pub base_url: url::Url,
}

impl Client {
    pub fn try_new(base_url: &str) -> Result<Self, url::ParseError> {
        Ok(Self {
            client: reqwest::Client::new(),
            base_url: url::Url::parse(base_url)?,
        })
    }

    pub async fn call_api<T: api::CoreRequest>(&self, request: &T) -> api::Result<T::Response> {
        request.send(&self.client, &self.base_url).await
    }

    pub async fn ping(&self) -> api::Result<api::ping::Response> {
        self.call_api(&api::ping::Request {}).await
    }
}

impl Default for Client {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: url::Url::parse("https://mecrm.dolylab.cc/api/v0.5/").unwrap(),
        }
    }
}
