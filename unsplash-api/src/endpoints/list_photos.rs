//! https://unsplash.com/documentation#list-photos

use http_api_client_endpoint::{
    http::{
        header::{ACCEPT, USER_AGENT},
        Method, StatusCode,
    },
    Body, Endpoint, Request, Response, MIME_APPLICATION_JSON,
};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use url::Url;

use crate::{
    endpoints::{
        common::{EndpointError, EndpointRet},
        helper::get_n_from_headers_by_key,
        URL_BASE, USER_AGENT_VALUE,
    },
    objects::{
        pagination::{Pagination, RESPONSE_HEADER_KEY_PER_PAGE, RESPONSE_HEADER_KEY_TOTAL},
        photo::Photo,
        rate_limiting::{
            RateLimiting, RESPONSE_HEADER_KEY_RATELIMIT_LIMIT,
            RESPONSE_HEADER_KEY_RATELIMIT_REMAINING,
        },
        ResponseBodyErrJson,
    },
};

#[derive(Debug, Clone)]
pub struct ListPhotos {
    pub access_key: String,
    //
    pub page: Option<usize>,
    pub per_page: Option<usize>,
    pub order_by: Option<ListPhotosOrderBy>,
}

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ListPhotosOrderBy {
    Latest,
    Oldest,
    Popular,
}

impl ListPhotos {
    pub fn new(access_key: impl AsRef<str>) -> Self {
        Self {
            access_key: access_key.as_ref().to_owned(),
            page: Default::default(),
            per_page: Default::default(),
            order_by: Default::default(),
        }
    }
}

impl Endpoint for ListPhotos {
    type RenderRequestError = EndpointError;

    type ParseResponseOutput =
        EndpointRet<(ListPhotosResponseBodyOkJson, Pagination, RateLimiting)>;
    type ParseResponseError = EndpointError;

    fn render_request(&self) -> Result<Request<Body>, Self::RenderRequestError> {
        let url = format!("{}/photos", URL_BASE);
        let mut url = Url::parse(url.as_str()).map_err(EndpointError::MakeRequestUrlFailed)?;

        url.query_pairs_mut()
            .append_pair("client_id", &self.access_key);

        if let Some(page) = &self.page {
            url.query_pairs_mut()
                .append_pair("page", page.to_string().as_str());
        }
        if let Some(per_page) = &self.per_page {
            url.query_pairs_mut()
                .append_pair("per_page", per_page.to_string().as_str());
        }
        if let Some(order_by) = &self.order_by {
            url.query_pairs_mut()
                .append_pair("order_by", order_by.to_string().as_str());
        }

        let request = Request::builder()
            .method(Method::GET)
            .uri(url.as_str())
            .header(USER_AGENT, USER_AGENT_VALUE)
            .header(ACCEPT, MIME_APPLICATION_JSON)
            .body(vec![])
            .map_err(EndpointError::MakeRequestFailed)?;

        Ok(request)
    }

    fn parse_response(
        &self,
        response: Response<Body>,
    ) -> Result<Self::ParseResponseOutput, Self::ParseResponseError> {
        let status = response.status();
        match status {
            StatusCode::OK => {
                let ok_json = serde_json::from_slice(response.body())
                    .map_err(EndpointError::DeResponseBodyOkJsonFailed)?;

                let pagination = Pagination {
                    total: get_n_from_headers_by_key(response.headers(), RESPONSE_HEADER_KEY_TOTAL)
                        .ok(),
                    per_page: get_n_from_headers_by_key(
                        response.headers(),
                        RESPONSE_HEADER_KEY_PER_PAGE,
                    )
                    .ok(),
                };
                let rate_limiting = RateLimiting {
                    limit: get_n_from_headers_by_key(
                        response.headers(),
                        RESPONSE_HEADER_KEY_RATELIMIT_LIMIT,
                    )
                    .ok(),
                    remaining: get_n_from_headers_by_key(
                        response.headers(),
                        RESPONSE_HEADER_KEY_RATELIMIT_REMAINING,
                    )
                    .ok(),
                };

                Ok(EndpointRet::Ok((ok_json, pagination, rate_limiting)))
            }
            StatusCode::TOO_MANY_REQUESTS => Ok(EndpointRet::RateLimitIsReached),
            status => match serde_json::from_slice::<ResponseBodyErrJson>(response.body()) {
                Ok(err_json) => Ok(EndpointRet::Other((status, Ok(err_json)))),
                Err(_) => Ok(EndpointRet::Other((
                    status,
                    Err(response.body().to_owned()),
                ))),
            },
        }
    }
}

//
//
//
pub type ListPhotosResponseBodyOkJson = Vec<Photo>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_by_to_string() {
        assert_eq!(ListPhotosOrderBy::Latest.to_string(), "latest");
    }

    #[test]
    fn de_response_body_ok_json() {
        match serde_json::from_str::<ListPhotosResponseBodyOkJson>(include_str!(
            "../../tests/response_body_json_files/list_photos_ok.json"
        )) {
            Ok(ok_json) => {
                assert_eq!(ok_json.len(), 10);
            }
            Err(err) => panic!("{}", err),
        }
    }
}
