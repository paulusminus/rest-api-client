use json_api_client::{Result, ApiClient, BasicAuthentication};

const PREFIX: &str = "https://lipl.paulmin.nl/api/v1/";
const USERNAME: &str = "paul";
const PASSWORD: &str = "CumGranoSalis";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let mut api_client = ApiClient::new(PREFIX, Some(BasicAuthentication::new(USERNAME, PASSWORD)));
    let lyrics = api_client.lyrics().await?;
    println!("{:#?}", lyrics);
    Ok(())
}
