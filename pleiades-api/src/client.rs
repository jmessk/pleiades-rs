use std::str;

use crate::api;

#[derive(Debug)]
pub struct Client {
    pub client: reqwest::Client,
    pub base_url: url::Url,
}

impl Client {
    pub fn try_new(base_url: &str) -> Result<Self, url::ParseError> {
        Ok(Self {
            client: reqwest::Client::new(),
            base_url: url::Url::parse(base_url)?,
        })
    }

    pub async fn call_api<T: api::CoreRequest>(&self, request: &T) -> api::Result<T::Response> {
        request.send(&self.client, &self.base_url).await
    }

    pub async fn ping(&self) -> api::Result<api::ping::Response> {
        self.call_api(&api::ping::Request {}).await
    }

    pub async fn generate_test_job(&self) -> api::Result<()> {
        let lambda = {
            let request = api::lambda::create::Request::builder()
                .data_id("1")
                .runtime("pleiades+example")
                .build();

            self.call_api(&request).await?
        };

        let create_job = {
            let request = api::job::create::Request::builder()
                .lambda_id(lambda.lambda_id)
                .data_id("1")
                .build();

            self.call_api(&request).await?
        };

        let job_info = {
            let request = api::job::info::Request::builder()
                .job_id(create_job.job_id)
                .except("Finished")
                .timeout(10)
                .build();

            self.call_api(&request).await?
        };

        let output = {
            let request = api::data::download::Request::builder()
                .data_id(job_info.output.unwrap().data_id)
                .build();

            self.call_api(&request).await?
        };

        Ok(())
    }

    pub async fn contract_test_job(&self) -> api::Result<()> {
        let register = {
            let request = api::worker::register::Request::builder()
                .runtimes(&["pleiades+example"])
                .build();

            self.call_api(&request).await?
        };

        let contract = {
            let request = api::worker::contract::Request::builder()
                .worker_id(register.worker_id)
                .timeout(10)
                .build();

            self.call_api(&request).await?
        };

        let job_id = match contract.job_id {
            Some(job_id) => job_id,
            None => {
                println!("no job contracted");
                return Ok(());
            }
        };

        let _update = {
            let request = api::job::update::Request::builder()
                .job_id(job_id)
                .data_id("1")
                .status("finished")
                .build();

            self.call_api(&request).await?
        };

        Ok(())
    }
}

impl Default for Client {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: url::Url::parse("https://mecrm.dolylab.cc/api/v0.5/").unwrap(),
        }
    }
}
