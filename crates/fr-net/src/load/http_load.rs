use std::time::Duration;

use ureq::{Agent, ResponseExt};
use url::Url;

use crate::load::net_error::NetError;
use crate::load::response::Response;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

pub fn http_load(url: &Url) -> Result<Response, NetError> {
    let agent: Agent = Agent::config_builder()
        .timeout_global(Some(REQUEST_TIMEOUT))
        .build()
        .into();
    let mut response = agent.get(url.as_str()).call().map_err(map_error)?;

    let status = response.status().as_u16();
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    let final_url = Url::parse(&response.get_uri().to_string()).unwrap_or_else(|_| url.clone());
    let body = response
        .body_mut()
        .read_to_vec()
        .map_err(|error| NetError::Http(error.to_string()))?;

    Ok(Response {
        url: final_url,
        status,
        content_type,
        body,
    })
}

fn map_error(error: ureq::Error) -> NetError {
    match error {
        ureq::Error::StatusCode(code) => NetError::Status(code),
        other => NetError::Http(other.to_string()),
    }
}
