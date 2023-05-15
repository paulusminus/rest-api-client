use crate::{Result};
use futures_util::{future::ready, TryFutureExt};
use reqwest::{Client, RequestBuilder, Response};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Clone)]
pub struct ApiClient {
    client: Client,
    prefix: String,
    authentication: Authentication,
}

#[derive(Clone, Default)]
pub enum Authentication {
    Basic(BasicAuthentication),
    Bearer(String),
    #[default]
    None,
}

impl Authentication {
    pub fn new_basic(username: &str, password: &str) -> Self {
        Authentication::Basic(
            BasicAuthentication::new(username, password)
        )
    }
    pub fn new_bearer(token: &str) -> Self {
        Authentication::Bearer(token.to_owned())
    }
}

#[derive(Clone)]
pub struct BasicAuthentication {
    username: String,
    password: String,
}

impl BasicAuthentication {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }
}

impl ApiClient {
    pub fn new(prefix: &str, authentication: Authentication) -> Self {
        Self {
            client: Client::builder().user_agent("Rest api client").build().unwrap(),
            prefix: prefix.into(),
            authentication,
        }
    }

    fn add_authentication<T>(&self, uri: &str, f: impl Fn(&Client, String) -> RequestBuilder, t: Option<T>) -> RequestBuilder
    where
        T: Serialize,
    {
        let mut builder = f(&self.client, self.uri(uri));
        builder = match &self.authentication {
            Authentication::Basic(basic) => 
                builder.basic_auth(
                    basic.username.clone(),
                    Some(basic.password.clone())
                )
            ,
            Authentication::Bearer(token) =>
                builder.bearer_auth(token)
            ,
            Authentication::None => builder,
        };

        if let Some(object) = t {
            builder = builder.json(&object)
        }

        builder
    }

    fn uri(&self, uri: &str) -> String {
        format!("{}{}", self.prefix, uri)
    }

    pub async fn delete(&self, uri: &str) -> Result<()> {
        self.add_authentication::<()>(uri, Client::delete, None)
            .send()
            .and_then(|r| ready(r.error_for_status()).map_ok(|_| ()))
            .await
    }

    pub async fn get<T>(&self, uri: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.add_authentication::<()>(uri, Client::get, None)
            .send()
            .and_then(Response::json::<T>)
            .await
    }

    pub async fn post<T, U>(&self, uri: &str, object: U) -> Result<T>
    where
        T: DeserializeOwned,
        U: Serialize,
    {
        self.add_authentication::<U>(uri, Client::post, Some(object))
            .send()
            .and_then(Response::json::<T>)
            .await
    }

    pub async fn put<T, U>(&self, uri: &str, object: U) -> Result<T>
    where
        T: DeserializeOwned,
        U: Serialize,
    {
        self.add_authentication::<U>(uri, Client::put, Some(object))
            .send()
            .and_then(Response::json::<T>)
            .await
    }
}
