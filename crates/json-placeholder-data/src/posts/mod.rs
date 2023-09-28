use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

const PLACEHOLDER_JSON_POSTS: &str = include_str!("posts.json");

#[skip_serializing_none]
#[derive(Deserialize, Serialize)]
pub struct Post {
    pub id: Option<i32>,
    pub title: String,
    pub body: String,
    #[serde(rename = "userId")]
    pub user_id: Option<i32>,
}

impl std::fmt::Display for Post {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Post: title = {}", self.title)
    }
}

/// Get all the posts available
///
/// # Example
/// ```
/// use json_placeholder_data::posts::get_all;
/// assert_eq!(get_all().len(), 100);
/// ```
pub fn get_all() -> Vec<Post> {
    serde_json::from_str(PLACEHOLDER_JSON_POSTS).unwrap()
}

/// Get a post by id
///
/// # Example
/// ```
/// use json_placeholder_data::posts::get;
/// assert_eq!(
///     get(1).title.as_str(),
///     "sunt aut facere repellat provident occaecati excepturi optio reprehenderit",
/// );
/// ```
pub fn get(id: i32) -> Post {
    get_all()
        .into_iter()
        .filter(|p| p.id == Some(id))
        .last()
        .unwrap()
}
