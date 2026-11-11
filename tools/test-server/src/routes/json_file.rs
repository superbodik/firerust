use crate::server::http_reply::HttpReply;

pub fn json_file() -> HttpReply {
    HttpReply::new(
        200,
        "application/json",
        b"{\"name\": \"firerust\", \"tabs\": [1, 2, 3]}".to_vec(),
    )
}
