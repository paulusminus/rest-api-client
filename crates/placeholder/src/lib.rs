use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

pub const PLACEHOLDER_BASE: &str = "https://jsonplaceholder.typicode.com/";

#[skip_serializing_none]
#[derive(Deserialize, Serialize)]
pub struct Post {
    pub id: Option<i32>,
    pub title: String,
    pub body: String,
    #[serde(rename = "userId")]
    pub user_id: Option<i32>,
}

impl Display for Post {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Post: title = {}", self.title)
    }
}
