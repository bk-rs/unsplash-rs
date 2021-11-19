use std::fmt;

use http_api_client_endpoint::{
    http::{Error as HttpError, StatusCode},
    Body,
};
use serde_json::Error as SerdeJsonError;
use url::ParseError as UrlParseError;

use crate::objects::ResponseBodyErrJson;

#[derive(Debug, Clone)]
pub enum EndpointRet<T>
where
    T: fmt::Debug + Clone,
{
    Ok(T),
    RateLimitIsReached,
    Other((StatusCode, Result<ResponseBodyErrJson, Body>)),
}

#[derive(thiserror::Error, Debug)]
pub enum EndpointError {
    #[error("MakeRequestUrlFailed {0}")]
    MakeRequestUrlFailed(UrlParseError),
    #[error("MakeRequestFailed {0}")]
    MakeRequestFailed(HttpError),
    #[error("DeResponseBodyOkJsonFailed {0}")]
    DeResponseBodyOkJsonFailed(SerdeJsonError),
}
