pub mod data;
pub mod error;
pub mod job;
pub mod kv;
pub mod lambda;
pub mod ping;
pub mod worker;

use std::borrow::Cow;

pub trait CoreRequest {
    type Response: CoreResponse;

    fn endpoint(&self) -> Cow<'static, str>;

    fn send(
        &self,
        client: &reqwest::Client,
        host: &url::Url,
    ) -> impl std::future::Future<Output = Result<Self::Response>> + Send;
}

pub trait CoreResponse {
    type Response: CoreResponse;

    fn from_response(
        response: reqwest::Response,
    ) -> impl std::future::Future<Output = Result<Self::Response>> + Send;
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to send request: {0}")]
    Request(#[from] reqwest::Error),

    #[error("failed to parse response: {0}")]
    Parse(#[from] serde_json::Error),

    #[error("MEC-RM internal error: {0}")]
    Response(error::Response),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl Error {
    pub fn parse(error: &str) -> Self {
        match serde_json::from_str::<error::Response>(error) {
            Ok(response) => Error::Response(response),
            Err(e) => Error::Parse(e),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
