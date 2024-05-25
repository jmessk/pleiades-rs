use crate::api::IntoBody;

pub struct UploadBlobBuilder {
    data: Vec<u8>,
}

impl UploadBlobBuilder {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn data(mut self, data: Vec<u8>) -> Self {
        self.data = data;
        self
    }
}

impl IntoBody for UploadBlobBuilder {
    fn into_body(self) -> reqwest::Body {
        let input = reqwest::multipart::Form::new()
            .part("data", reqwest::multipart::Part::bytes(self.data));
        // reqwest::Body::from(input.stream())
        reqwest::Client::new().post("url").multipart(input)
    }
}

#[derive(serde::Deserialize)]
pub struct UploadBlobResponse {
    data_id: String,
}

impl UploadBlobResponse {
    pub fn new(data_id: String) -> Self {
        Self { data_id }
    }

    pub fn data_id(&self) -> &str {
        &self.data_id
    }
}
