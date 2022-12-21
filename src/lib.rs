mod api_client;
mod error;

pub use error::Error;
pub use api_client::ApiClient;
pub type Result<T> = std::result::Result<T, Error>;
