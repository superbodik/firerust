use boa_engine::{Context, JsError};

use crate::runtime::js_failure::JsFailure;

pub fn describe_error(error: JsError, context: &mut Context) -> JsFailure {
    let Ok(opaque) = error.into_opaque(context) else {
        return JsFailure::new("script error");
    };
    match opaque.to_string(context) {
        Ok(text) => JsFailure::new(text.to_std_string_escaped()),
        Err(_) => JsFailure::new("script error"),
    }
}
