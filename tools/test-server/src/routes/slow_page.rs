use std::thread::sleep;
use std::time::Duration;

use crate::server::http_reply::HttpReply;

const DELAY: Duration = Duration::from_secs(3);

pub fn slow_page() -> HttpReply {
    sleep(DELAY);
    HttpReply::html(
        "<!DOCTYPE html><html><head><title>Slow</title></head><body><h1>Slow page finished</h1></body></html>",
    )
}
