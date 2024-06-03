mod data;
mod error;
mod job;
mod kv;
mod lambda;
mod worker;

use anyhow::Result;
use reqwest::IntoUrl;
use std::sync::Arc;

trait MECRMRequest {
    async fn send<U: IntoUrl>(
        self,
        client: Arc<reqwest::Client>,
        host: U,
    ) -> Result<impl MECRMResponse>;
}

trait MECRMResponse {}
