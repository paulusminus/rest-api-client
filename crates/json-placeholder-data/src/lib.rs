pub mod albums;
pub mod comments;
pub mod photos;
pub mod posts;
pub mod todos;
pub mod users;

pub const PLACEHOLDER_BASE: &str = "https://jsonplaceholder.typicode.com/";

#[macro_export]
macro_rules! from_json {
    ($data:expr) => {
        serde_json::from_str($data).unwrap()
    };
}

#[macro_export]
macro_rules! by_id {
    ($id:expr) => {
        get_all()
            .into_iter()
            .filter(|data| data.id == Some($id))
            .last()
            .unwrap()
    };
}
