mod api_client;
mod error;

pub use error::Error;
pub use api_client::{ApiClient, BasicAuthentication};
pub type Result<T> = reqwest::Result<T>;
