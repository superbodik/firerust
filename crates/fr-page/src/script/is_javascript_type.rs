pub fn is_javascript_type(declared: Option<&str>) -> bool {
    let Some(declared) = declared else {
        return true;
    };
    let normalized = declared.trim().to_ascii_lowercase();
    matches!(
        normalized.as_str(),
        "" | "text/javascript"
            | "application/javascript"
            | "application/x-javascript"
            | "text/ecmascript"
            | "application/ecmascript"
            | "text/jscript"
    )
}
