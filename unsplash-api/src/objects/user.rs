use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub links: UserLinks,
    pub profile_image: UserProfileImage,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserLinks {
    pub html: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserProfileImage {
    pub large: Url,
}
