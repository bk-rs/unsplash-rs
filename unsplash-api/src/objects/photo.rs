use serde::{Deserialize, Serialize};
use url::Url;

use crate::objects::user::User;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Photo {
    pub id: String,
    pub width: usize,
    pub height: usize,
    pub color: String,
    pub blur_hash: Option<String>,
    pub urls: PhotoUrls,
    pub user: User,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PhotoUrls {
    pub raw: Url,
    pub full: Url,
    pub regular: Url,
    pub small: Url,
    pub thumb: Url,
}
