use http_api_client_endpoint::http::{HeaderMap, HeaderValue};

pub(super) fn get_n_from_headers_by_key(
    headers: &HeaderMap<HeaderValue>,
    key: &str,
) -> Result<usize, String> {
    if let Some(value) = headers.get(key) {
        if let Ok(str) = value.to_str() {
            if let Ok(n) = str.parse::<usize>() {
                Ok(n)
            } else {
                Err(format!("ValueInvalid key: [{}] value: [{}]", key, str))
            }
        } else {
            Err(format!(
                "ValueInvalid key: [{}] value: [{:?}]",
                key,
                value.as_bytes()
            ))
        }
    } else {
        Err(format!("KeyNotFound key: [{}]", key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_n_from_headers_by_key() {
        let mut map = HeaderMap::new();
        map.insert("x-foo", "1".parse().unwrap());
        assert_eq!(get_n_from_headers_by_key(&map, "x-foo"), Ok(1));
    }
}
