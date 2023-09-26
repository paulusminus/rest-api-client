Library to simplify calls to a RESTful API using a JSON file as the data source.
GET, POST, PUT and DELETE methods are supported.


# Example

Getting a list of posts from [Json Placeholder](https://jsonplaceholder.typicode.com/)

```rust
# use json_api_client::{ApiClientBuilder, Authentication, Error};
# use placeholder::Post;
#
# tokio_test::block_on(async {
    let base = "https://jsonplaceholder.typicode.com/";
    let posts = ApiClientBuilder::new(base)
        .build()?
        .get::<Vec<Post>>("posts")
        .await?;

#     assert_eq!(posts.len(), 100);
#     Ok::<(), Error>(())
# });
```