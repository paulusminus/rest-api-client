use crate::{Result};
use futures_util::TryFutureExt;
use reqwest::{Client, RequestBuilder};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Clone)]
pub struct ApiClient {
    client: Client,
    _prefix: String,
    basic_authentication: Option<BasicAuthentication>,
}

#[derive(Clone)]
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

    fn add_authentication<T>(&self, uri: &str, f: impl Fn(&Client, String) -> RequestBuilder, t: Option<T>) -> RequestBuilder
    where
        T: Serialize,
    {
        let mut builder = f(&self.client, self.uri(uri));
        if let Some(auth) = &self.basic_authentication {
            builder = builder.basic_auth(auth.username.clone(), Some(auth.password.clone()));
        }
        if let Some(object) = t {
            builder = builder.json(&object)
        }

        builder
    }

    fn uri(&self, uri: &str) -> String {
        format!("{}{}", self._prefix, uri)
    }

    pub async fn delete(&self, uri: &str) -> Result<()> {
        let response = self.add_authentication::<()>(uri, Client::delete, None).send().await?;
        response.error_for_status().map(|_| ())
    }

    pub async fn get<T>(&self, uri: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.add_authentication::<()>(uri, Client::get, None)
            .send()
            .and_then(|r| r.json::<T>())
            .await
    }

    pub async fn post<T, U>(&self, uri: &str, object: U) -> Result<T>
    where
        T: DeserializeOwned,
        U: Serialize,
    {
        self.add_authentication::<U>(uri, Client::post, Some(object))
            .send()
            .and_then(|r| r.json::<T>())
            .await
    }

    pub async fn put<T, U>(&self, uri: &str, object: U) -> Result<T>
    where
        T: DeserializeOwned,
        U: Serialize,
    {
        self.add_authentication::<U>(uri, Client::put, Some(object))
            .send()
            .and_then(|r| r.json::<T>())
            .await
    }


}
