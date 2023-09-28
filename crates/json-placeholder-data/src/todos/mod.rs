use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

const PLACEHOLDER_JSON_TODOS: &str = include_str!("todos.json");

#[skip_serializing_none]
#[derive(Deserialize, Serialize)]
pub struct Todo {
    #[serde(rename = "userId")]
    pub user_id: i32,
    pub id: Option<i32>,
    pub title: String,
    pub completed: bool,
}

/// Get all the todos available
/// 
/// # Example 
/// ```
/// use json_placeholder_data::todos::get_all;
/// assert_eq!(get_all().len(), 200);
/// ```
pub fn get_all() -> Vec<Todo> {
    serde_json::from_str(PLACEHOLDER_JSON_TODOS).unwrap()
}

/// Get a todo by id
/// 
/// # Example 
/// ```
/// use json_placeholder_data::todos::get;
/// assert_eq!(
///     get(67).title.as_str(),
///     "quia voluptatibus voluptatem quos similique maiores repellat",
/// );
/// ```
pub fn get(id: i32) -> Todo {
    get_all().into_iter().filter(|todo| todo.id == Some(id)).last().unwrap()
}