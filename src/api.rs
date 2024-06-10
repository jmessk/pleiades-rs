mod data;
mod error;
mod job;
mod kv;
mod lambda;
mod worker;

use anyhow::Result;
use std::sync::Arc;

trait MecrmRequest {
    type Response: MecrmResponse;
    async fn request(self, client: Arc<reqwest::Client>, host: url::Url) -> Result<Self::Response>;
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
