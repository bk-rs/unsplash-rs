//! https://unsplash.com/documentation#content-safety
//! https://unsplash.com/documentation#search-photos

use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ContentFilter {
    Low,
    High,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        assert_eq!(ContentFilter::Low.to_string(), "low");
    }
}
