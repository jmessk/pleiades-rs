mod data;
mod error;
mod job;
mod kv;
mod lambda;
mod worker;

use anyhow::Result;
use std::sync::Arc;

trait MecrmRequest {
    async fn request(
        self,
        client: Arc<reqwest::Client>,
        host: url::Url,
    ) -> Result<impl MecrmResponse>;
}

trait MecrmResponse {
    async fn from_response(response: reqwest::Response) -> Result<impl MecrmResponse>;
}
