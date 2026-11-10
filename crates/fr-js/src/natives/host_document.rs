use boa_engine::{Context, JsNativeError, JsResult};
use fr_dom::tree::shared_document::SharedDocument;

pub fn host_document(context: &Context) -> JsResult<SharedDocument> {
    context
        .get_data::<SharedDocument>()
        .cloned()
        .ok_or_else(|| {
            JsNativeError::typ()
                .with_message("no document is attached to this runtime")
                .into()
        })
}
