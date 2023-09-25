#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use futures_util::{future::ready, TryFutureExt};
use reqwest::{Client, RequestBuilder, Response, Result};
pub use reqwest::Error;
use serde::{de::DeserializeOwned, Serialize};

/// Before one can do any api request, an ApiClient must be constructed
pub struct ApiClient {
    client: Client,
    prefix: String,
    authentication: Authentication,
}

/// This library support two ways of authentication
/// Either Basic of Bearer
#[derive(Default)]
pub enum Authentication {
    Basic(BasicAuthentication),
    Bearer(String),
    #[default]
    None,
}

impl Authentication {
    pub fn new_basic(username: &str, password: &str) -> Self {
        Authentication::Basic(BasicAuthentication::new(username, password))
    }
    pub fn new_bearer(token: &str) -> Self {
        Authentication::Bearer(token.to_owned())
    }
}

pub struct BasicAuthentication {
    username: String,
    password: String,
}

impl BasicAuthentication {
    /// Create a new instance of BasicAuthentication with provided username and password
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }
}

impl ApiClient {
    /// Try to create a new instance of ApiClient
    /// 
    /// # Example
    /// 
    /// ```
    /// # use json_api_client::{ApiClient, Authentication, Error};
    /// # use placeholder::{PLACEHOLDER_BASE};
    /// let api_client = ApiClient::try_new(PLACEHOLDER_BASE, Authentication::None, None)?;
    /// # Ok::<(), Error>(())
    /// ```
    pub fn try_new(prefix: &str, authentication: Authentication, user_agent: Option<&str>) -> Result<Self> {
        Client::builder()
            .user_agent(user_agent.unwrap_or("Rest json api client"))
            .build()
            .map(|client| Self {
                client,
                prefix: prefix.into(),
                authentication,
            })
    }

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
            Authentication::Bearer(token) => builder.bearer_auth(token),
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
    /// # use json_api_client::{ApiClient, Authentication, Error};
    /// # use placeholder::Post;
    /// #
    /// # tokio_test::block_on(async {
    ///     let base = "https://jsonplaceholder.typicode.com/";
    ///     ApiClient::try_new(base, Authentication::None, None)?
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
    /// # use json_api_client::{ApiClient, Authentication, Error};
    /// # use placeholder::Post;
    /// #
    /// # tokio_test::block_on(async {
    ///     let base = "https://jsonplaceholder.typicode.com/";
    ///     let posts = ApiClient::try_new(base, Authentication::None, None)?
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
    /// # use json_api_client::{ApiClient, Authentication, Error};
    /// # use placeholder::Post;
    /// #
    /// # tokio_test::block_on(async {
    ///     let base = "https://jsonplaceholder.typicode.com/";
    ///     let post = ApiClient::try_new(base, Authentication::None, None)?
    ///         .get::<Post>("posts/1")
    ///         .await?;
    ///
    /// #       assert_eq!(post.user_id, Some(1));
    /// #       Ok::<(), Error>(())
    /// # });
    /// ```
    pub async fn get<T>(&self, uri: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.create_request::<()>(uri, Client::get, None)
            .send()
            .and_then(Response::json::<T>)
            .await
    }

    /// # Example 
    /// 
    /// Try to create a new post on [Json Placeholder](https://jsonplaceholder.typicode.com/)
    /// If successful the created post is returned
    /// 
    /// ```
    /// # use json_api_client::{ApiClient, Authentication, Error};
    /// # use placeholder::Post;
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
    ///     let post = ApiClient::try_new(base, Authentication::None, None)?
    ///         .post::<Post, _>("posts", new_post)
    ///         .await?;
    ///
    /// #     assert_eq!(post.user_id, Some(34));
    /// #     Ok::<(), Error>(())
    /// # });
    /// ```
    pub async fn post<T, U>(&self, uri: &str, object: U) -> Result<T>
    where
        T: DeserializeOwned,
        U: Serialize,
    {
        self.create_request::<U>(uri, Client::post, Some(object))
            .send()
            .and_then(Response::json::<T>)
            .await
    }

    /// # Example 
    /// 
    /// Try to change a post with specific id on [Json Placeholder](https://jsonplaceholder.typicode.com/)
    /// If successful the changed post is returned
    /// 
    /// ```
    /// # use json_api_client::{ApiClient, Authentication, Error};
    /// # use placeholder::Post;
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
    ///     let post = ApiClient::try_new(base, Authentication::None, None)?
    ///         .put::<Post, _>("posts/1", changed_post)
    ///         .await?;
    ///
    /// #     assert_eq!(post.user_id, Some(34));
    /// #     Ok::<(), Error>(())
    /// # });
    /// ```
    pub async fn put<T, U>(&self, uri: &str, object: U) -> Result<T>
    where
        T: DeserializeOwned,
        U: Serialize,
    {
        self.create_request::<U>(uri, Client::put, Some(object))
            .send()
            .and_then(Response::json::<T>)
            .await
    }
}
