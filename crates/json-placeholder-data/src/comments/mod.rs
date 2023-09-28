use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

const PLACEHOLDER_JSON_COMMENTS: &str = include_str!("comments.json");

#[skip_serializing_none]
#[derive(Deserialize, Serialize)]
pub struct Comment {
    #[serde(rename = "postId")]
    pub post_id: i32,
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub body: String,
}

/// Get all the comments available
/// 
/// # Example 
/// ```
/// use json_placeholder_data::comments::get_all;
/// assert_eq!(get_all().len(), 500);
/// ```
pub fn get_all() -> Vec<Comment> {
    serde_json::from_str(PLACEHOLDER_JSON_COMMENTS).unwrap()
}

/// Get a comment by id
/// 
/// # Example 
/// ```
/// use json_placeholder_data::comments::get;
/// assert_eq!(
///     get(37).email.as_str(),
///     "Jacky@victoria.net",
/// );
/// ```
pub fn get(id: i32) -> Comment {
    get_all().into_iter().filter(|p| p.id == Some(id)).last().unwrap()
}