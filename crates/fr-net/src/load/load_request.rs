use crate::load::file_load::file_load;
use crate::load::http_load::http_load;
use crate::load::net_error::NetError;
use crate::load::request::Request;
use crate::load::response::Response;

pub fn load_request(request: &Request) -> Result<Response, NetError> {
    match request.url.scheme() {
        "file" => file_load(&request.url),
        "http" | "https" => http_load(request),
        other => Err(NetError::UnsupportedScheme(other.to_owned())),
    }
}
