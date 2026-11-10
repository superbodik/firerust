use url::Url;

use crate::load::request_method::RequestMethod;

#[derive(Debug, Clone)]
pub struct Request {
    pub method: RequestMethod,
    pub url: Url,
    pub body: Vec<u8>,
    pub content_type: Option<String>,
}

impl Request {
    pub fn get(url: Url) -> Self {
        Self {
            method: RequestMethod::Get,
            url,
            body: Vec::new(),
            content_type: None,
        }
    }

    pub fn post(url: Url, body: Vec<u8>, content_type: &str) -> Self {
        Self {
            method: RequestMethod::Post,
            url,
            body,
            content_type: Some(content_type.to_owned()),
        }
    }
}
