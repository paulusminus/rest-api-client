use crate::{Error, Result};
use hyper::client::HttpConnector;
use hyper::{Body, client::Client, Request};
use hyper::body::{aggregate, Buf};
use hyper::http::HeaderValue;
use hyper::service::Service;
use hyper::header::USER_AGENT;
use hyper_rustls::HttpsConnector;
use lipl_core::Lyric;
use tower_http::set_header::{SetRequestHeader, request::SetRequestHeaderLayer};
use tower_http::trace::{TraceLayer, Trace};
use tower_http::classify::{StatusInRangeAsFailures, SharedClassifier};
use tower_http::decompression::{DecompressionLayer, Decompression};
use tower_http::auth::{AddAuthorizationLayer, AddAuthorization};
use tower::{ServiceBuilder};

const USERNAME: &str = "paul";
const PASSWORD: &str = "CumGranoSalis";
const URL: &str = "https://lipl.paulmin.nl/api/v1/lyric?full=true";

type ApiService = Trace<SetRequestHeader<AddAuthorization<Decompression<Client<HttpsConnector<HttpConnector>>>>, HeaderValue>, SharedClassifier<StatusInRangeAsFailures>>;

fn connector() -> HttpsConnector<HttpConnector> {
    hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_or_http()
        .enable_http1()
        .build()
}

fn service() -> ApiService {
    ServiceBuilder::new()
        .layer(TraceLayer::new(
            StatusInRangeAsFailures::new(400..=599).into_make_classifier()
        ))
        .layer(SetRequestHeaderLayer::overriding(
            USER_AGENT,
            HeaderValue::from_static("tower-http-demo")
        ))
        .layer(AddAuthorizationLayer::basic(USERNAME, PASSWORD))
        .layer(DecompressionLayer::new())
        .service(Client::builder().build(connector()))
}

pub struct ApiClient {
    service: ApiService,
    _prefix: String,
}

impl ApiClient {
    pub fn new(prefix: &str) -> Self {
        Self {
            service: service(),
            _prefix: prefix.to_owned(),
        }
    }

    pub async fn lyrics(&mut self) -> Result<Vec<Lyric>> {
        let request = Request::get(URL).body(Body::empty())?;
        let response = self.service.call(request).await.map_err(|_| Error::Call)?;
        let body = response.into_body();
        let a = aggregate(body).await.map_err(|_| Error::Aggregate)?;
        
        let lyrics: Vec<lipl_core::Lyric> = serde_json::from_reader(a.reader())?;
        Ok(lyrics)
    }
}
