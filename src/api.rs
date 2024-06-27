mod data;
mod error;
mod job;
mod kv;
mod lambda;
mod worker;

pub use data::{download::DataDownloadRequest, upload::DataUploadRequest};
pub use job::{create::JobCreateRequest, info::JobInfoRequest, update::JobUpdateRequest};
pub use lambda::create::LambdaCreateRequest;
pub use worker::{contract::WorkerContractRequest, register::WorkerRegisterRequest};

use anyhow::Result;

pub trait MecrmRequest {
    type Response: MecrmResponse;

    fn endpoint(&self) -> String;

    fn send(
        &self,
        client: &reqwest::Client,
        host: &url::Url,
    ) -> impl std::future::Future<Output = Result<Self::Response>> + Send;
}

pub trait MecrmResponse {
    type Response: MecrmResponse;
    fn from_response(
        response: reqwest::Response,
    ) -> impl std::future::Future<Output = Result<Self::Response>> + Send;
}
