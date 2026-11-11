use fr_download::save::downloads_folder::downloads_folder;
use fr_download::save::save_download::save_download;
use fr_net::load::response::Response;

pub fn finish_download(response: &Response, requested_from_disk: bool) -> String {
    if requested_from_disk {
        return "firerust cannot show this kind of file".to_owned();
    }
    match save_download(response, &downloads_folder()) {
        Ok(path) => {
            let name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("the file");
            format!("Saved {name} to your Downloads folder")
        }
        Err(error) => format!("Download failed: {error}"),
    }
}
