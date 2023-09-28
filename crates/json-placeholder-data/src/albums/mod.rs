use crate::{by_id, from_json};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

const PLACEHOLDER_JSON_COMMENTS: &str = include_str!("albums.json");

#[skip_serializing_none]
#[derive(Deserialize, Serialize)]
pub struct Album {
    #[serde(rename = "userId")]
    pub user_id: i32,
    pub id: Option<i32>,
    pub title: String,
}

/// Get all the albums available
///
/// # Example
/// ```
/// use json_placeholder_data::albums::get_all;
/// assert_eq!(get_all().len(), 100);
/// ```
pub fn get_all() -> Vec<Album> {
    from_json!(PLACEHOLDER_JSON_COMMENTS)
}

/// Get an album by id
///
/// # Example
/// ```
/// use json_placeholder_data::albums::get;
/// assert_eq!(
///     get(100).title.as_str(),
///     "enim repellat iste",
/// );
/// ```
pub fn get(id: i32) -> Album {
    by_id!(id)
}
