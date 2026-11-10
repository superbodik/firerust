use url::Url;

use crate::load::file_load::file_load;
use crate::load::http_load::http_load;
use crate::load::net_error::NetError;
use crate::load::response::Response;

pub fn load_url(url: &Url) -> Result<Response, NetError> {
    match url.scheme() {
        "file" => file_load(url),
        "http" | "https" => http_load(url),
        other => Err(NetError::UnsupportedScheme(other.to_owned())),
    }
}
