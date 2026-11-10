use url::Url;

pub fn same_origin(first: &Url, second: &Url) -> bool {
    let origin = first.origin();
    origin.is_tuple() && origin == second.origin()
}
