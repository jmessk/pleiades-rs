use crate::api::{Error, CoreResponse, Result};

#[derive(serde::Deserialize, Debug)]
pub struct Response {
    pub code: i32,
    pub status: String,
    pub message: String,
}

impl CoreResponse for Response {
    type Response = Response;

    async fn from_response(response: reqwest::Response) -> Result<Response> {
        match response.json::<Response>().await {
            Ok(response) => Ok(response),
            Err(e) => Err(Error::Request(e)),
        }
        // Ok(response.json::<ErrorResponse>().await.unwrap())
    }
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{code: {}, status: {}, message: {}}}",
            self.code, self.status, self.message
        )
    }
}
