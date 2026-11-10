use boa_engine::JsValue;
use fr_dom::node::node_id::NodeId;

pub fn id_value(id: Option<NodeId>) -> JsValue {
    match id {
        Some(id) => JsValue::from(id.index() as f64),
        None => JsValue::from(-1.0),
    }
}
