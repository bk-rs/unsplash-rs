//! https://unsplash.com/documentation#error-messages

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ErrorMessages {
    pub errors: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn de() {
        match serde_json::from_str::<ErrorMessages>(include_str!(
            "../../tests/response_body_json_files/err_invalid_access_token.json"
        )) {
            Ok(_err_json) => {}
            Err(err) => panic!("{}", err),
        }
    }
}
