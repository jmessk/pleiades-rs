use crate::api::{Error, Response, Result};

#[derive(serde::Deserialize, Debug)]
pub struct ErrorResponse {
    pub code: i32,
    pub status: String,
    pub message: String,
}

impl Response for ErrorResponse {
    type Response = ErrorResponse;

    async fn from_response(response: reqwest::Response) -> Result<ErrorResponse> {
        match response.json::<ErrorResponse>().await {
            Ok(response) => Ok(response),
            Err(e) => Err(Error::Request(e)),
        }
        // Ok(response.json::<ErrorResponse>().await.unwrap())
    }
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{code: {}, status: {}, message: {}}}",
            self.code, self.status, self.message
        )
    }
}
