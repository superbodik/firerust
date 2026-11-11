use crate::server::http_reply::HttpReply;

pub fn links_page() -> HttpReply {
    HttpReply::html(
        "<!DOCTYPE html><html><head><title>Links</title>\
         <style>body { font-family: sans-serif; margin: 24px } a { display: block; margin: 8px 0 }\
         .tall { height: 900px; background-color: #eeeeee }</style></head><body>\
         <h1 id=\"top\">Link kinds</h1>\
         <a href=\"#details\">Jump to the details (same page)</a>\
         <a href=\"/plain\">Plain page (same tab)</a>\
         <a href=\"/plain\" target=\"_blank\">Plain page in a new tab</a>\
         <a href=\"/notes.txt\">A text file</a>\
         <a href=\"/data.json\">A JSON file</a>\
         <a href=\"/picture.png\">A picture</a>\
         <a href=\"/archive.zip\">A zip archive (downloads)</a>\
         <a href=\"/report\">A file with a Content-Disposition name (downloads)</a>\
         <a href=\"mailto:team@example.com?subject=Hello\">Write an email</a>\
         <a href=\"javascript:alert(1)\">A javascript link (ignored)</a>         <a href=\"#\" onclick=\"location.href = '/plain'; return false\">Script: location.href</a>         <a href=\"#\" onclick=\"window.open('/notes.txt'); return false\">Script: window.open</a>\
         <div class=\"tall\">space</div>\
         <h2 id=\"details\">The details</h2>\
         <p>You jumped here.</p>\
         <a href=\"#top\">Back to the top</a>\
         <div class=\"tall\">more space</div>\
         </body></html>",
    )
}
