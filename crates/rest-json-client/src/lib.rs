#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use futures_util::{TryFutureExt, future::ready};
pub use reqwest::Error;
use reqwest::{Client, RequestBuilder, Response, Result};
use serde::{Serialize, de::DeserializeOwned};

const DEFAULT_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"));

/// Before one can do any api request, an ApiClient must be constructed
pub struct ApiClient {
    client: Client,
    prefix: String,
    authentication: Authentication,
}

pub struct ApiClientBuilder {
    prefix: String,
    authentication: Authentication,
    user_agent: Option<String>,
}

impl ApiClientBuilder {
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_owned(),
            authentication: Authentication::default(),
            user_agent: None,
        }
    }
    pub fn authentication(&mut self, auth: Authentication) -> &mut Self {
        self.authentication = auth;
        self
    }
    pub fn user_agent(&mut self, user_agent: &str) -> &mut Self {
        self.user_agent = Some(user_agent.to_owned());
        self
    }
    pub fn build(&self) -> Result<ApiClient> {
        Client::builder()
            .user_agent(
                self.user_agent
                    .clone()
                    .unwrap_or(DEFAULT_USER_AGENT.to_owned()),
            )
            .build()
            .map(|client| ApiClient {
                authentication: self.authentication.clone(),
                client,
                prefix: self.prefix.clone(),
            })
    }
}
/// This library support two ways of authentication
/// Either Basic of Bearer
#[derive(Clone, Default)]
pub enum Authentication {
    Basic(BasicAuthentication),
    Bearer(Option<String>),
    #[default]
    None,
}

impl Authentication {
    pub fn new_basic(username: &str, password: &str) -> Self {
        Authentication::Basic(BasicAuthentication::new(username, password))
    }
    pub fn new_bearer(token: &str) -> Self {
        Authentication::Bearer(Some(token.to_owned()))
    }
}

#[derive(Clone)]
pub struct BasicAuthentication {
    username: String,
    password: String,
}

impl BasicAuthentication {
    /// Create a new instance of BasicAuthentication with provided username and password
    pub fn new<S: Into<String>>(username: S, password: S) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }
}

impl ApiClient {
    fn create_request<T>(
        &self,
        uri: &str,
        f: impl Fn(&Client, String) -> RequestBuilder,
        t: Option<T>,
    ) -> RequestBuilder
    where
        T: Serialize,
    {
        let mut builder = f(&self.client, self.uri(uri));
        builder = match &self.authentication {
            Authentication::Basic(basic) => {
                builder.basic_auth(basic.username.clone(), Some(basic.password.clone()))
            }
            Authentication::Bearer(token) => match token {
                Some(t) => builder.bearer_auth(t.clone()),
                None => builder,
            },
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

    /// # Example
    ///
    /// Try to delete a post with specific id from [Json Placeholder](https://jsonplaceholder.typicode.com/)
    ///
    /// ```
    /// # use rest_json_client::{ApiClientBuilder, Authentication, Error};
    /// # use json_placeholder_data::posts::Post;
    /// #
    /// # tokio_test::block_on(async {
    ///     let base = "https://jsonplaceholder.typicode.com/";
    ///     ApiClientBuilder::new(base)
    ///         .build()?
    ///         .delete("posts/1")
    ///         .await?;
    ///
    /// #       Ok::<(), Error>(())
    /// # });
    /// ```
    pub async fn delete(&self, uri: &str) -> Result<()> {
        self.create_request::<()>(uri, Client::delete, None)
            .send()
            .and_then(|r| ready(r.error_for_status()).map_ok(|_| ()))
            .await
    }

    /// # Example 1
    ///
    /// Try to return a list of posts from [JsonPlaceholder](https://jsonplaceholder.typicode.com/)
    ///
    /// ```rust
    /// # use rest_json_client::{ApiClientBuilder, Authentication, Error};
    /// # use json_placeholder_data::posts::Post;
    /// #
    /// # tokio_test::block_on(async {
    ///     let base = "https://jsonplaceholder.typicode.com/";
    ///     let posts = ApiClientBuilder::new(base)
    ///         .build()?
    ///         .get::<Vec<Post>>("posts")
    ///         .await?;
    ///
    /// #       assert_eq!(posts.len(), 100);
    /// #       Ok::<(), Error>(())
    /// # });
    /// ```
    ///
    /// # Example 2
    ///
    /// Try to return a single post with specific id from [Json Placeholder](https://jsonplaceholder.typicode.com/)
    ///
    /// ```rust
    /// # use rest_json_client::{ApiClientBuilder, Authentication, Error};
    /// # use json_placeholder_data::posts::Post;
    /// #
    /// # tokio_test::block_on(async {
    ///     let base = "https://jsonplaceholder.typicode.com/";
    ///     let post = ApiClientBuilder::new(base)
    ///         .build()?
    ///         .get::<Post>("posts/1")
    ///         .await?;
    ///
    /// #       assert_eq!(post.user_id, Some(1));
    /// #       Ok::<(), Error>(())
    /// # });
    /// ```
    pub async fn get<R>(&self, uri: &str) -> Result<R>
    where
        R: DeserializeOwned,
    {
        self.create_request::<()>(uri, Client::get, None)
            .send()
            .and_then(Response::json::<R>)
            .await
    }

    /// # Example
    ///
    /// Try to create a new post on [Json Placeholder](https://jsonplaceholder.typicode.com/)
    /// If successful the created post is returned
    ///
    /// ```
    /// # use rest_json_client::{ApiClientBuilder, Authentication, Error};
    /// # use json_placeholder_data::posts::Post;
    /// #
    /// # tokio_test::block_on(async {
    ///
    ///     let new_post = Post {
    ///         id: None,
    ///         title: "Hallo".to_owned(),
    ///         body: "Hallo".to_owned(),
    ///         user_id: Some(34),
    ///     };
    ///     let base = "https://jsonplaceholder.typicode.com/";
    ///     let post = ApiClientBuilder::new(base)
    ///         .build()?
    ///         .post::<_, Post>("posts", new_post)
    ///         .await?;
    ///
    /// #     assert_eq!(post.user_id, Some(34));
    /// #     Ok::<(), Error>(())
    /// # });
    /// ```
    pub async fn post<T, R>(&self, uri: &str, object: T) -> Result<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        self.create_request::<T>(uri, Client::post, Some(object))
            .send()
            .and_then(Response::json::<R>)
            .await
    }

    /// use post_validation to get a Json Web Token
    pub async fn token_request<T>(&mut self, uri: &str, signature: &str, object: T) -> Result<()>
    where
        T: Serialize,
    {
        let token = self
            .create_request::<T>(uri, Client::post, Some(object))
            .header("Signature", signature)
            .send()
            .and_then(Response::text)
            .await?;
        self.authentication = Authentication::Bearer(Some(token));
        Ok(())
    }

    /// # Example
    ///
    /// Try to change a post with specific id on [Json Placeholder](https://jsonplaceholder.typicode.com/)
    /// If successful the changed post is returned
    ///
    /// ```
    /// # use rest_json_client::{ApiClientBuilder, Authentication, Error};
    /// # use json_placeholder_data::posts::Post;
    /// #
    /// # tokio_test::block_on(async {
    ///
    ///     let changed_post = Post {
    ///         id: None,
    ///         title: "Hallo".to_owned(),
    ///         body: "Hallo".to_owned(),
    ///         user_id: Some(34),
    ///     };
    ///     let base = "https://jsonplaceholder.typicode.com/";
    ///     let post = ApiClientBuilder::new(base)
    ///         .build()?
    ///         .put::<_, Post>("posts/1", changed_post)
    ///         .await?;
    ///
    /// #     assert_eq!(post.user_id, Some(34));
    /// #     Ok::<(), Error>(())
    /// # });
    /// ```
    pub async fn put<T, R>(&self, uri: &str, object: T) -> Result<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        self.create_request::<T>(uri, Client::put, Some(object))
            .send()
            .and_then(Response::json::<R>)
            .await
    }
}
