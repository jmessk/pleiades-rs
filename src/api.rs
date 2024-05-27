mod error;
mod data;
mod job;
mod kv;
mod lambda;
mod worker;

use anyhow::Result;
use std::sync::Arc;

trait MECRMRequest {
    async fn send(self, client: Arc<reqwest::Client>) -> Result<impl MECRMResponse>;
}

trait MECRMResponse {}
