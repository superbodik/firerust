use url::Url;

use crate::load::guess_content_type::guess_content_type;
use crate::load::net_error::NetError;
use crate::load::response::Response;

pub fn file_load(url: &Url) -> Result<Response, NetError> {
    let path = url.to_file_path().map_err(|()| NetError::InvalidFilePath)?;
    let body = std::fs::read(&path).map_err(NetError::FileRead)?;
    Ok(Response {
        url: url.clone(),
        status: 200,
        content_type: guess_content_type(&path),
        body,
    })
}
