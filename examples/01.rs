use json_api_client::{Result, ApiClient};

const PREFIX: &str = "https://lipl.paulmin.nl/api/v1/";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let mut api_client = ApiClient::new(PREFIX);
    let lyrics = api_client.lyrics().await?;
    println!("{:#?}", lyrics);
    Ok(())
}
