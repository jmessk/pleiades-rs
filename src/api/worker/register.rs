use anyhow::{Context, Result};
use std::borrow::Cow;
use std::sync::Arc;

use crate::api::{MecrmRequest, MecrmResponse};
use crate::Client;

#[derive(serde::Serialize, Debug, typed_builder::TypedBuilder)]
pub struct WorkerRegisterRequest<'a> {
    #[serde(rename = "runtime")]
    runtimes: Vec<Cow<'a, str>>,
}

impl<'a> MecrmRequest for WorkerRegisterRequest<'a> {
    type Response = WorkerRegisterResponse;

    fn endpoint(&self, host: &url::Url) -> url::Url {
        host.join("worker").unwrap()
    }

    async fn send(&self, client: &Arc<Client>) -> Result<WorkerRegisterResponse> {
        let response = client
            .client()
            .post(self.endpoint(client.host()))
            .json(&self)
            .send()
            .await?;

        WorkerRegisterResponse::from_response(response).await
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct WorkerRegisterResponse {
    pub code: u32,
    pub status: String,
    #[serde(rename = "id")]
    pub worker_id: String,
    #[serde(rename = "runtime")]
    pub runtimes: Vec<String>,
}

impl MecrmResponse for WorkerRegisterResponse {
    type Response = WorkerRegisterResponse;

    async fn from_response(response: reqwest::Response) -> Result<WorkerRegisterResponse> {
        response
            .json()
            .await
            .with_context(|| "failed to register worker")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Client;

    #[tokio::test]
    async fn test_worker_register() {
        let client = Client::builder()
            .host("https://mecrm.dolylab.cc/api/v0.5-snapshot/")
            .build();

        let client = Arc::new(client);

        let request = WorkerRegisterRequest::builder()
            .runtimes(vec!["test1".into(), "test2".into()])
            .build();

        dbg!(&request);

        let response = request.send(&client).await;
        assert!(response.is_ok());

        dbg!(response.unwrap().runtimes);
    }
}
