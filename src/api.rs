mod data;
mod error;
mod job;
mod kv;
mod lambda;
mod worker;

pub use data::{download::DataDownloadRequest, upload::DataUploadRequest};
pub use error::ErrorResponse;
pub use job::{create::JobCreateRequest, info::JobInfoRequest, update::JobUpdateRequest};
pub use lambda::create::LambdaCreateRequest;
pub use worker::{contract::WorkerContractRequest, register::WorkerRegisterRequest};

pub trait Request {
    type Response: Response;

    fn endpoint(&self) -> String;

    fn send(
        &self,
        client: &reqwest::Client,
        host: &url::Url,
    ) -> impl std::future::Future<Output = Result<Self::Response>> + Send;
}

pub trait Response {
    type Response: Response;
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
    Response(ErrorResponse),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
