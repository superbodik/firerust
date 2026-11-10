use boa_engine::{JsError, JsNativeError};
use fr_dom::error::dom_error::DomError;

pub fn dom_error_to_js(error: DomError) -> JsError {
    JsNativeError::error()
        .with_message(error.to_string())
        .into()
}
