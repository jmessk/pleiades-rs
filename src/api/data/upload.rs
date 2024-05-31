use getset::Getters;

use crate::api::*;

#[derive(serde::Serialize)]
pub struct UploadBlobBuilder {
    data: Vec<u8>,
}

impl UploadBlobBuilder {
    const ENDPOINT: &'static str = "/data";
}

impl MECRMRequest for UploadBlobBuilder {
    async fn send(self, client: Arc<reqwest::Client>, host: url::Url) -> Result<UploadBlobResponse> {
        let multipart = reqwest::multipart::Form::new()
            .part("file", reqwest::multipart::Part::bytes(self.data).file_name("data"));

        let response = client.post(Self::ENDPOINT)
            .multipart(multipart).

        unimplemented!()
    }
}

#[derive(serde::Deserialize)]
pub struct UploadBlobResponse {
    code: i32,
    status: String,
    data_id: String,
    checksum: String,
}

impl MECRMResponse for UploadBlobResponse {}
