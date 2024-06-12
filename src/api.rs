mod data;
mod error;
mod job;
mod kv;
mod lambda;
mod worker;

use anyhow::Result;

trait MecrmRequest {
    type Response: MecrmResponse;
    async fn send(&self) -> Result<Self::Response>;
}

trait MecrmResponse {
    type Response: MecrmResponse;
    async fn from_response(response: reqwest::Response) -> Result<Self::Response>;
}

// pub enum ApiResult<T>
// where
//     T: MecrmResponse,
// {
//     Success(T),
//     Error(T),
// }
