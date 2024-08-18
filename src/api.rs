mod data;
mod error;
mod job;
pub mod kv;
mod lambda;
mod worker;

use std::borrow::Cow;

pub use data::{
    download::{DataDownloadRequest, DataDownloadResponse},
    upload::{DataUploadRequest, DataUploadResponse},
};
pub use error::ErrorResponse;
pub use job::{
    create::{JobCreateRequest, JobCreateResponse},
    info::{JobInfoRequest, JobInfoResponse},
    update::{JobUpdateRequest, JobUpdateResponse},
};
pub use lambda::create::{LambdaCreateRequest, LambdaCreateResponse};
pub use worker::{
    contract::{WorkerContractRequest, WorkerContractResponse},
    register::{WorkerRegisterRequest, WorkerRegisterResponse},
};

pub trait Request {
    type Response: Response;

    fn endpoint(&self) -> Cow<'static, str>;

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

pub type Result<T> = std::result::Result<T, Error>;

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
