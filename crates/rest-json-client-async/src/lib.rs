use hyper::body::Bytes;
use hyper_rustls::HttpsConnector;
use hyper_util::{client::legacy::{connect::HttpConnector, Client}, rt::TokioExecutor};
use http_body_util::Full;

pub fn create_client() -> Client<HttpsConnector<HttpConnector>, Full<Bytes>> {
    let https = hyper_rustls::HttpsConnectorBuilder::new().with_webpki_roots().https_only().enable_http1().build();
    Client::builder(TokioExecutor::new()).build(https)
}


#[cfg(test)]
mod tests {
    use std::io::read_to_string;
    use hyper::{body::Buf, header::HeaderValue, Request, StatusCode};
    use http_body_util::{BodyExt, Full};

    use super::create_client;

    const URI: &str = "https://www.paulmin.nl";

    #[tokio::test]
    async fn client() {
        let client = create_client();
        let request = Request::builder().uri(URI).body(Full::default()).unwrap();
        let response = client.request(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers().get("Content-Type"), Some(&HeaderValue::from_str("text/html").unwrap()));

        let body = response.into_body();
        let collected = body.collect().await.unwrap();
        let buf = collected.aggregate();
        let reader = buf.reader();
        
        let html = read_to_string(reader).unwrap();
        assert!(html.starts_with("<!doctype html>"));
    }

}
