use ureq::ResponseExt;
use url::Url;

use crate::load::net_error::NetError;
use crate::load::request::Request;
use crate::load::request_method::RequestMethod;
use crate::load::response::Response;
use crate::load::shared_agent::shared_agent;

const FORM_CONTENT_TYPE: &str = "application/x-www-form-urlencoded";
const MAX_BODY_BYTES: u64 = 256 * 1024 * 1024;

pub fn http_load(request: &Request) -> Result<Response, NetError> {
    let agent = shared_agent();
    let outcome = match request.method {
        RequestMethod::Get => agent.get(request.url.as_str()).call(),
        RequestMethod::Post => {
            let content_type = request.content_type.as_deref().unwrap_or(FORM_CONTENT_TYPE);
            agent
                .post(request.url.as_str())
                .header("Content-Type", content_type)
                .send(&request.body[..])
        }
    };
    let mut response = outcome.map_err(|error| NetError::Http(error.to_string()))?;
    let status = response.status().as_u16();
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    let content_disposition = response
        .headers()
        .get("content-disposition")
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    let final_url =
        Url::parse(&response.get_uri().to_string()).unwrap_or_else(|_| request.url.clone());
    let body = response
        .body_mut()
        .with_config()
        .limit(MAX_BODY_BYTES)
        .read_to_vec()
        .map_err(|error| NetError::Http(error.to_string()))?;
    Ok(Response {
        url: final_url,
        status,
        content_type,
        content_disposition,
        body,
    })
}
