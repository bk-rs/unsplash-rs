//! https://unsplash.com/documentation#rate-limiting

use serde::{Deserialize, Serialize};

pub const RESPONSE_HEADER_KEY_RATELIMIT_LIMIT: &str = "x-ratelimit-limit";
pub const RESPONSE_HEADER_KEY_RATELIMIT_REMAINING: &str = "x-ratelimit-remaining";

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub struct RateLimiting {
    pub limit: Option<usize>,
    pub remaining: Option<usize>,
}
