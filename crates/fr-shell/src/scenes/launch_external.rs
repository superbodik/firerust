use std::io;
use std::process::Command;

use url::Url;

pub fn launch_external(url: &Url) -> io::Result<()> {
    Command::new("explorer.exe")
        .arg(url.as_str())
        .spawn()
        .map(|_| ())
}
