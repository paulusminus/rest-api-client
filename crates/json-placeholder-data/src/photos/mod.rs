use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

const PLACEHOLDER_JSON_PHOTOS: &str = include_str!("photos.json");

#[skip_serializing_none]
#[derive(Deserialize, Serialize)]
pub struct Photo {
    #[serde(rename = "albumId")]
    pub album_id: i32,
    pub id: Option<i32>,
    pub title: String,
    pub url: String,
    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url: String,
}

/// Get all the photos available
///
/// # Example
/// ```
/// use json_placeholder_data::photos::get_all;
/// assert_eq!(get_all().len(), 5000);
/// ```
pub fn get_all() -> Vec<Photo> {
    serde_json::from_str::<Vec<Photo>>(PLACEHOLDER_JSON_PHOTOS).unwrap()
}

/// Get a photo by id
///
/// # Example
/// ```
/// use json_placeholder_data::photos::get;
/// assert_eq!(
///     get(3421).url.as_str(),
///     "https://via.placeholder.com/600/b13cb8",
/// );
/// ```
pub fn get(id: i32) -> Photo {
    get_all()
        .into_iter()
        .filter(|photo| photo.id == Some(id))
        .last()
        .unwrap()
}
