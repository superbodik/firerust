use std::cell::RefCell;

use boa_engine::{Context, JsResult, JsValue};

use crate::natives::string_arg::string_arg;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigationRequest {
    pub address: String,
    pub in_new_tab: bool,
}

thread_local! {
    static REQUEST: RefCell<Option<NavigationRequest>> = const { RefCell::new(None) };
}

pub fn navigate_request(
    _this: &JsValue,
    args: &[JsValue],
    context: &mut Context,
) -> JsResult<JsValue> {
    let address = string_arg(args, 0, context)?;
    let in_new_tab = args.get(1).is_some_and(JsValue::to_boolean);
    REQUEST.with(|slot| {
        *slot.borrow_mut() = Some(NavigationRequest {
            address,
            in_new_tab,
        })
    });
    Ok(JsValue::undefined())
}

pub fn take_navigation_request() -> Option<NavigationRequest> {
    REQUEST.with(|slot| slot.borrow_mut().take())
}
