use crate::{Result};
use lipl_core::Lyric;
use reqwest::{Client, RequestBuilder};

const URL: &str = "https://lipl.paulmin.nl/api/v1/lyric?full=true";

pub struct ApiClient {
    client: Client,
    _prefix: String,
    basic_authentication: Option<BasicAuthentication>,
}

pub struct BasicAuthentication {
    username: String,
    password: String,
}

impl BasicAuthentication {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_owned(),
            password: password.to_owned(),
        }
    }
}

impl ApiClient {
    pub fn new(prefix: &str, basic_authentication: Option<BasicAuthentication>) -> Self {
        Self {
            client: Client::builder().user_agent("Rest api client").build().unwrap(),
            _prefix: prefix.to_owned(),
            basic_authentication,
        }
    }

    fn add_authentication(&self, request_builder: RequestBuilder) -> RequestBuilder {
        match &self.basic_authentication {
            Some(auth) => request_builder.basic_auth(auth.username.clone(), Some(auth.password.clone())),
            None => request_builder,
        }
    }

    pub async fn lyrics(&self) -> Result<Vec<Lyric>> {
        self.add_authentication(self.client.get(URL))
            .send()
            .await?
            .json::<Vec<Lyric>>()
            .await
    }
}
