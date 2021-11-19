//! https://unsplash.com/documentation#pagination-headers

use serde::{Deserialize, Serialize};

pub const RESPONSE_HEADER_KEY_TOTAL: &str = "x-total";
pub const RESPONSE_HEADER_KEY_PER_PAGE: &str = "x-per-page";

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub struct Pagination {
    pub total: Option<usize>,
    pub per_page: Option<usize>,
}

pub const MAX_PER_PAGE: usize = 30;
