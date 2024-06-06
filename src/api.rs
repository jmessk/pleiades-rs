mod data;
mod error;
mod job;
mod kv;
mod lambda;
mod worker;

use anyhow::Result;
use std::sync::Arc;

trait MecrmRequest<T>
where
    T: MecrmResponse,
{
    fn endpoint(&self) -> url::Url;
    async fn request(self, client: Arc<reqwest::Client>, host: url::Url) -> Result<ApiResult<T>>;
}

trait MecrmResponse {
    async fn from_response(response: reqwest::Response) -> Result<impl MecrmResponse>;
}

pub enum ApiResult<T>
where
    T: MecrmResponse,
{
    Success(T),
    Error(T),
}
