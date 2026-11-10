use url::Url;

use crate::load::load_request::load_request;
use crate::load::net_error::NetError;
use crate::load::request::Request;
use crate::load::response::Response;

pub fn load_url(url: &Url) -> Result<Response, NetError> {
    let response = load_request(&Request::get(url.clone()))?;
    if response.status >= 400 {
        return Err(NetError::Status(response.status));
    }
    Ok(response)
}
