use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

const PLACEHOLDER_JSON_USERS: &str = include_str!("users.json");

#[skip_serializing_none]
#[derive(Deserialize, Serialize)]
pub struct Geo {
    pub lat: String,
    pub lng: String,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize)]
pub struct Address {
    pub street: String,
    pub suite: String,
    pub city: String,
    pub zipcode: String,
    pub geo: Geo,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize)]
pub struct Company {
    pub name: String,
    #[serde(rename = "catchPhrase")]
    pub catch_phrase: String,
    pub bs: String,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub username: String,
    pub email: String,
    pub address: Address,
    pub phone: String,
    pub website: String,
    pub company: Company,
}

/// Get all the users available
///
/// # Example
/// ```
/// use json_placeholder_data::users::get_all;
/// assert_eq!(get_all().len(), 10);
/// ```
pub fn get_all() -> Vec<User> {
    serde_json::from_str(PLACEHOLDER_JSON_USERS).unwrap()
}

/// Get a user by id
///
/// # Example
/// ```
/// use json_placeholder_data::users::get;
/// assert_eq!(
///     get(1).name.as_str(),
///     "Leanne Graham",
/// );
/// ```
pub fn get(id: i32) -> User {
    get_all()
        .into_iter()
        .filter(|u| u.id == Some(id))
        .last()
        .unwrap()
}
