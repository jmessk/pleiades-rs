use bytes::Bytes;
use reqwest::multipart::{Form, Part};
use std::borrow::Cow;

use crate::api::{Error, ErrorResponse, Request, Response, Result};

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
pub struct DataUploadRequest {
    /// byte data to upload
    #[builder(setter(into))]
    // data: Cow<'static, [u8]>,
    pub data: Bytes,
}

impl Request for DataUploadRequest {
    type Response = DataUploadResponse;

    fn endpoint(&self) -> Cow<'static, str> {
        "data".into()
    }

    async fn send(&self, client: &reqwest::Client, host: &url::Url) -> Result<DataUploadResponse> {
        let endpoint = host.join(&self.endpoint()).unwrap();

        let form = {
            let part = Part::bytes(self.data.to_vec()).file_name("data");
            Form::new().part("file", part)
        };

        let request = client.post(endpoint).multipart(form).build()?;

        tracing::debug!("uploading data: {} bytes", self.data.len());

        let response = client.execute(request).await?;
        DataUploadResponse::from_response(response).await
    }
}

/// Response from uploading data
#[derive(serde::Deserialize, Debug)]
pub struct DataUploadResponse {
    pub code: u32,
    pub status: String,

    /// uploaded blob data ID
    #[serde(rename = "id")]
    pub data_id: String,
    pub checksum: String,
}

impl Response for DataUploadResponse {
    type Response = DataUploadResponse;

    async fn from_response(response: reqwest::Response) -> Result<DataUploadResponse> {
        let body = response.text().await?;

        match serde_json::from_str::<DataUploadResponse>(&body) {
            Ok(response) => {
                tracing::info!("data uploaded");
                tracing::debug!("data uploaded: {:?}", response);

                Ok(response)
            }
            Err(_) => {
                tracing::error!("failed to upload data: {}", body);

                match serde_json::from_str::<ErrorResponse>(&body) {
                    Ok(response) => Err(Error::Response(response)),
                    Err(e) => Err(Error::Parse(e)),
                }
            }
        }
    }
}
