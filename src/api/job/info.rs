use std::borrow::Cow;

use crate::api::{Error, ErrorResponse, Request, Response, Result};

/// Request to get job info
///
/// # Example
///
/// ```rust
/// use mecrs::api::job::info::JobInfoRequest;
///
/// let request = JobInfoRequest::builder()
///    .job_id("1")
///    .build();
///
/// let response = request.send(&client).await?;
/// let job_id = response.job_id;
///
/// // `except` and `timeout` can be used together
/// // requester can poll until job status changes
/// let request_with_except = JobInfoRequest::builder()
///     .job_id("1")
///     .except("Finished") // wait until job status is not Finished up to timeout
///     .timeout(10)        // timeout in seconds
///     .build();
/// ```
#[derive(Debug, typed_builder::TypedBuilder)]
pub struct JobInfoRequest<'a> {
    /// job ID to get info
    #[builder(setter(into))]
    job_id: Cow<'a, str>,

    /// except job status. use with timeout
    #[builder(default, setter(strip_option, into))]
    except: Option<Cow<'a, str>>,

    /// job info request timeout. use with except
    #[builder(default, setter(strip_option))]
    timeout: Option<u32>,
}

impl<'a> Request for JobInfoRequest<'a> {
    type Response = JobInfoResponse;

    fn endpoint(&self) -> String {
        format!("job/{}", self.job_id)
    }

    async fn send(&self, client: &reqwest::Client, host: &url::Url) -> Result<JobInfoResponse> {
        log::debug!("getting job info: {:?}", self);

        let endpoint = host.join(&self.endpoint()).unwrap();
        let response = match &self.except {
            Some(except) => {
                client
                    .get(endpoint)
                    .query(&[("except", except)])
                    .query(&[("timeout", self.timeout)])
                    .send()
                    .await?
            }
            None => client.get(endpoint).send().await?,
        };

        JobInfoResponse::from_response(response).await
    }
}

/// Response from getting job info
#[derive(serde::Deserialize, Debug)]
pub struct Lambda {
    /// lambda ID to execute the job
    #[serde(rename = "id")]
    pub lambda_id: String,

    /// runtime that supports the lambda
    pub runtime: String,

    /// blob data ID as lambda code
    #[serde(rename = "codex")]
    pub data_id: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Input {
    /// blob data ID as job input
    #[serde(rename = "id")]
    pub data_id: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct Output {
    /// blob data ID as job output
    #[serde(rename = "id")]
    pub data_id: String,
}

/// Response from getting job info
///
/// # Example
///
/// ```rust
/// use mecrs::api::job::info::JobInfoRequest;
///
/// let request = JobInfoRequest::builder()
///     .job_id("1")
///     .build();
///
/// let response = request.send(&client).await?;
///
/// let job_id = response.job_id;
/// let lambda_id = response.lambda.lambda_id;
/// let input_id = response.input.data_id;
/// ```
#[derive(serde::Deserialize, Debug)]
pub struct JobInfoResponse {
    pub code: i32,
    pub status: String,

    /// job ID
    #[serde(rename = "id")]
    pub job_id: String,

    /// job status
    #[serde(rename = "state")]
    pub job_status: String,
    pub lambda: Lambda,
    pub input: Input,
    pub output: Option<Output>,
}

impl Response for JobInfoResponse {
    type Response = JobInfoResponse;

    async fn from_response(response: reqwest::Response) -> Result<JobInfoResponse> {
        let body = response.text().await?;

        match serde_json::from_str::<JobInfoResponse>(&body) {
            Ok(response) => {
                log::info!("job info fetched");
                log::debug!("job info fetched: {}", body);

                Ok(response)
            }
            Err(_) => match serde_json::from_str::<ErrorResponse>(&body) {
                Ok(response) => {
                    log::error!("failed to fetch job info: {:?}", response);
                    Err(Error::Response(response))
                }
                Err(e) => Err(Error::Parse(e)),
            },
        }
    }
}
