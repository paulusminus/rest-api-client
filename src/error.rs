use thiserror::Error;


#[derive(Debug, Error)]
pub enum Error {
    #[error("Call")]
    Call,

    #[error("Aggregate")]
    Aggregate,

    #[error("Json: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Request: {0}")]
    Request(#[from] hyper::http::Error),
}