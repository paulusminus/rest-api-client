mod api_client;

pub type Result<T> = reqwest::Result<T>;
pub use api_client::{ApiClient, Authentication, BasicAuthentication};
pub use reqwest::Error;
