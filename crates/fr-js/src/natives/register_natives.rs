use boa_engine::object::ObjectInitializer;
use boa_engine::property::Attribute;
use boa_engine::{Context, JsResult, NativeFunction, js_string};

use crate::natives::console_write::console_write;
use crate::natives::dom_append_child::dom_append_child;
use crate::natives::dom_attributes::dom_attributes;
use crate::natives::dom_child_nodes::dom_child_nodes;
use crate::natives::dom_create_element::dom_create_element;
use crate::natives::dom_create_text::dom_create_text;
use crate::natives::dom_get_attribute::dom_get_attribute;
use crate::natives::dom_get_element_by_id::dom_get_element_by_id;
use crate::natives::dom_insert_before::dom_insert_before;
use crate::natives::dom_kind::dom_kind;
use crate::natives::dom_matches::dom_matches;
use crate::natives::dom_parent::dom_parent;
use crate::natives::dom_query_all::dom_query_all;
use crate::natives::dom_remove_attribute::dom_remove_attribute;
use crate::natives::dom_remove_child::dom_remove_child;
use crate::natives::dom_set_attribute::dom_set_attribute;
use crate::natives::dom_set_inner_html::dom_set_inner_html;
use crate::natives::dom_set_text_content::dom_set_text_content;
use crate::natives::dom_tag_name::dom_tag_name;
use crate::natives::dom_text_content::dom_text_content;

pub fn register_natives(context: &mut Context) -> JsResult<()> {
    let mut builder = ObjectInitializer::new(context);
    builder
        .function(NativeFunction::from_fn_ptr(dom_kind), js_string!("kind"), 1)
        .function(
            NativeFunction::from_fn_ptr(dom_tag_name),
            js_string!("tagName"),
            1,
        )
        .function(
            NativeFunction::from_fn_ptr(dom_parent),
            js_string!("parent"),
            1,
        )
        .function(
            NativeFunction::from_fn_ptr(dom_child_nodes),
            js_string!("childNodes"),
            1,
        )
        .function(
            NativeFunction::from_fn_ptr(dom_get_attribute),
            js_string!("getAttribute"),
            2,
        )
        .function(
            NativeFunction::from_fn_ptr(dom_attributes),
            js_string!("attributes"),
            1,
        )
        .function(
            NativeFunction::from_fn_ptr(dom_set_attribute),
            js_string!("setAttribute"),
            3,
        )
        .function(
            NativeFunction::from_fn_ptr(dom_remove_attribute),
            js_string!("removeAttribute"),
            2,
        )
        .function(
            NativeFunction::from_fn_ptr(dom_text_content),
            js_string!("textContent"),
            1,
        )
        .function(
            NativeFunction::from_fn_ptr(dom_set_text_content),
            js_string!("setTextContent"),
            2,
        )
        .function(
            NativeFunction::from_fn_ptr(dom_set_inner_html),
            js_string!("setInnerHtml"),
            2,
        )
        .function(
            NativeFunction::from_fn_ptr(dom_create_element),
            js_string!("createElement"),
            1,
        )
        .function(
            NativeFunction::from_fn_ptr(dom_create_text),
            js_string!("createText"),
            1,
        )
        .function(
            NativeFunction::from_fn_ptr(dom_append_child),
            js_string!("appendChild"),
            2,
        )
        .function(
            NativeFunction::from_fn_ptr(dom_insert_before),
            js_string!("insertBefore"),
            3,
        )
        .function(
            NativeFunction::from_fn_ptr(dom_remove_child),
            js_string!("removeChild"),
            1,
        )
        .function(
            NativeFunction::from_fn_ptr(dom_get_element_by_id),
            js_string!("getElementById"),
            1,
        )
        .function(
            NativeFunction::from_fn_ptr(dom_query_all),
            js_string!("queryAll"),
            2,
        )
        .function(
            NativeFunction::from_fn_ptr(dom_matches),
            js_string!("matches"),
            2,
        )
        .function(
            NativeFunction::from_fn_ptr(console_write),
            js_string!("log"),
            2,
        );
    let natives = builder.build();
    context.register_global_property(js_string!("__fr"), natives, Attribute::all())
}
