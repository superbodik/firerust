use fr_icons::catalog::icon::Icon;
use url::Url;

pub fn tab_icon(url: &Url) -> Icon {
    match url.scheme() {
        "file" => Icon::FileEarmark,
        "about" => Icon::House,
        _ => Icon::Globe,
    }
}
