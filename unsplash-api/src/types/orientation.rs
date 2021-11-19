//! https://unsplash.com/documentation#search-photos

use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Orientation {
    Landscape,
    Portrait,
    Squarish,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        assert_eq!(Orientation::Landscape.to_string(), "landscape");
    }
}
