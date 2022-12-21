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

fn use_authentication(request_builder: RequestBuilder, auth: Option<BasicAuthentication>) -> RequestBuilder {
    match auth {
        Some(a) => request_builder.basic_auth(a.username, Some(a.password)),
        None => request_builder,
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

    pub async fn lyrics(&mut self) -> Result<Vec<Lyric>> {
        use_authentication(self.client.get(URL), self.basic_authentication.take())
            .send()
            .await?
            .json::<Vec<Lyric>>()
            .await
    }
}
