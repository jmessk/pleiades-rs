mod data;
mod error;
mod job;
mod kv;
mod lambda;
mod worker;

use anyhow::Result;
use std::sync::Arc;

use crate::Client;

trait MecrmRequest {
    type Response: MecrmResponse;
    async fn send(self, client: Arc<Client>) -> Result<Self::Response>;
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
