//! https://unsplash.com/documentation#search-photos

use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    BlackAndWhite,
    Black,
    // TODO
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        assert_eq!(Color::BlackAndWhite.to_string(), "black_and_white");
        assert_eq!(Color::Black.to_string(), "black");
    }
}
