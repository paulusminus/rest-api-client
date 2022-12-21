use thiserror::Error;


#[derive(Debug, Error)]
pub enum Error {
    #[error("Call")]
    Call,

    #[error("Aggregate")]
    Aggregate,

}