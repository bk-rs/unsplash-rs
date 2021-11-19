pub mod common;
mod helper;
pub mod list_photos;
pub mod search_photos;

//
pub type CollectionId = usize;

//
pub const URL_BASE: &str = "https://api.unsplash.com";
pub const USER_AGENT_VALUE: &str = "curl/7.79.1";
