use bytes::BytesMut;
use hyper::{Body, client::Client, Request};
use hyper::body::HttpBody;
use tower_http::{decompression::Decompression};
use tower::{BoxError, Service, ServiceExt};


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut client = Decompression::new(Client::new());

    let request = Request::new(Body::empty());

    let response = client
        .ready()
        .await?
        .call(request)
        .await?;

// Read the body
    let mut body = response.into_body();
    let mut bytes = BytesMut::new();
    while let Some(chunk) = body.data().await {
        let chunk = chunk?;
        bytes.extend_from_slice(&chunk[..]);
    }
    let body = String::from_utf8(bytes.to_vec()).map_err(Into::<BoxError>::into)?;

    assert_eq!(body, "Hello, World!");
    Ok(())
}
