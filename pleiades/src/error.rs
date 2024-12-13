use pleiades_api::api::ApiError;

#[derive(thiserror::Error, Debug)]
pub enum PleiadesError {
    #[error("request failed: {0}")]
    Api(#[from] ApiError),
}
