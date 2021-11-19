//! https://unsplash.com/documentation#supported-languages

use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Lang {
    AF,
    // TODO
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        assert_eq!(Lang::AF.to_string(), "af");
    }
}
