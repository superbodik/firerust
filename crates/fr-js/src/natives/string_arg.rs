use boa_engine::{Context, JsResult, JsValue};

pub fn string_arg(args: &[JsValue], position: usize, context: &mut Context) -> JsResult<String> {
    let value = args.get(position).cloned().unwrap_or_default();
    Ok(value.to_string(context)?.to_std_string_escaped())
}
