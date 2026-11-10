use std::net::TcpStream;

use crate::routes::route::route;
use crate::server::read_request::read_request;
use crate::server::write_reply::write_reply;

pub fn serve_connection(mut stream: TcpStream) {
    let Some(request) = read_request(&stream) else {
        return;
    };
    let reply = route(&request);
    write_reply(&mut stream, &reply);
}
