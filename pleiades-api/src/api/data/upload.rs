use bytes::Bytes;
use reqwest::multipart::{Form, Part};
use std::borrow::Cow;

use crate::api::{ApiRequest, ApiResponse, ApiError, Result};

/// Request to upload byte data
///
/// # Example
///
/// ```rust
/// use mecrs::api::data::upload::DataUploadRequest;
///
/// let request = DataUploadRequest::builder()
///    .data(b"hello world")
///    .build();
///
/// let response = request.send(&client).await?;
/// let data_id = response.data_id;
/// ```
#[derive(Debug, typed_builder::TypedBuilder)]
pub struct Request {
    /// byte data to upload
    #[builder(setter(into))]
    // data: Cow<'static, [u8]>,
    pub data: Bytes,
}

impl ApiRequest for Request {
    type Response = Response;

    fn endpoint(&self) -> Cow<'static, str> {
        "data".into()
    }

    async fn send(&self, client: &reqwest::Client, host: &url::Url) -> Result<Response> {
        let endpoint = host.join(&self.endpoint()).unwrap();
        let form = {
            let part = Part::bytes(self.data.to_vec()).file_name("data");
            Form::new().part("file", part)
        };

        let request = client.post(endpoint).multipart(form).build()?;
        tracing::debug!("uploading data: {} bytes", self.data.len());

        let response = client.execute(request).await?;
        Response::from_response(response).await
    }
}

/// Response from uploading data
#[derive(serde::Deserialize, Debug)]
pub struct Response {
    pub code: u32,
    pub status: String,

    /// uploaded blob data ID
    #[serde(rename = "id")]
    pub data_id: String,
    pub checksum: String,
}

impl ApiResponse for Response {
    type Response = Response;

    async fn from_response(response: reqwest::Response) -> Result<Response> {
        let body = response.text().await?;

        match serde_json::from_str::<Response>(&body) {
            Ok(response) => {
                tracing::info!("data uploaded");
                tracing::debug!("data uploaded: {:?}", response);
                Ok(response)
            }
            Err(_) => {
                tracing::error!("failed to upload data: {}", body);
                Err(ApiError::parse(&body))
            }
        }
    }
}
