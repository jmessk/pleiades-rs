use getset::Getters;

use crate::api::*;

#[derive(serde::Serialize)]
pub struct UploadBlobBuilder {
    data: Vec<u8>,
}

impl MECRMRequest for UploadBlobBuilder {
    async fn send(&self, client: Arc<reqwest::Client>) -> Result<impl MECRMResponse> {
    }
}

#[derive(serde::Deserialize)]
pub struct UploadBlobResponse {
    code: i32,
    status: String,
    data_id: String,
    checksum: String,
}

impl UploadBlobResponse {
    pub fn data_id(&self) -> &str {
        &self.data_id
    }
}
