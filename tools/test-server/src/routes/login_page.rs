use crate::server::http_reply::HttpReply;

pub fn login_page() -> HttpReply {
    HttpReply::html(
        "<!DOCTYPE html><html><head><title>Log in</title></head><body><h1>Log in</h1>\
         <form action=\"/login\" method=\"post\">\
         <p>User <input type=\"text\" name=\"user\"></p>\
         <p>Password <input type=\"password\" name=\"password\"></p>\
         <p><input type=\"submit\" value=\"Log in\"></p>\
         </form></body></html>",
    )
}
