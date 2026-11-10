use std::fmt;

use boa_engine::object::ObjectInitializer;
use boa_engine::property::Attribute;
use boa_engine::{Context, JsString, JsValue, Source, js_string};
use fr_dom::node::node_id::NodeId;
use fr_dom::tree::shared_document::SharedDocument;

use crate::clock::now_ms::now_ms;
use crate::natives::console_write::take_console_lines;
use crate::natives::register_natives::register_natives;
use crate::runtime::describe_error::describe_error;
use crate::runtime::js_failure::JsFailure;

const PRELUDE: &str = include_str!("../prelude/prelude.js");

pub struct JsRuntime {
    context: Context,
}

impl fmt::Debug for JsRuntime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("JsRuntime").finish_non_exhaustive()
    }
}

impl JsRuntime {
    pub fn new(document: SharedDocument, address: &str) -> Result<Self, JsFailure> {
        Self::with_start_time(document, address, now_ms())
    }

    pub fn with_start_time(
        document: SharedDocument,
        address: &str,
        start_ms: f64,
    ) -> Result<Self, JsFailure> {
        let mut context = Context::default();
        context.insert_data(document);
        if let Err(error) = register_natives(&mut context) {
            return Err(describe_error(error, &mut context));
        }
        if let Err(error) = context.eval(Source::from_bytes(PRELUDE)) {
            return Err(describe_error(error, &mut context));
        }
        let mut runtime = Self { context };
        runtime.call_host("init", &[JsValue::from(JsString::from(address))])?;
        runtime.advance_time(start_ms)?;
        Ok(runtime)
    }

    pub fn run_script(&mut self, source: &str) -> Result<(), JsFailure> {
        let outcome = self.context.eval(Source::from_bytes(source));
        self.settle();
        match outcome {
            Ok(_) => Ok(()),
            Err(error) => Err(describe_error(error, &mut self.context)),
        }
    }

    pub fn dispatch_event(
        &mut self,
        node: NodeId,
        event_type: &str,
        point: Option<(f32, f32)>,
    ) -> Result<bool, JsFailure> {
        let init = match point {
            Some((x, y)) => ObjectInitializer::new(&mut self.context)
                .property(js_string!("clientX"), f64::from(x), Attribute::all())
                .property(js_string!("clientY"), f64::from(y), Attribute::all())
                .build()
                .into(),
            None => JsValue::undefined(),
        };
        let arguments = [
            JsValue::from(node.index() as f64),
            JsValue::from(JsString::from(event_type)),
            init,
        ];
        let result = self.call_host("dispatch", &arguments)?;
        Ok(result.as_boolean().unwrap_or(true))
    }

    pub fn fire_event(&mut self, event_type: &str) -> Result<(), JsFailure> {
        self.call_host("fire", &[JsValue::from(JsString::from(event_type))])?;
        Ok(())
    }

    pub fn advance_time(&mut self, now_ms: f64) -> Result<(), JsFailure> {
        self.call_host("advance", &[JsValue::from(now_ms)])?;
        Ok(())
    }

    pub fn next_timer_due(&mut self) -> Option<f64> {
        self.call_host("nextTimer", &[])
            .ok()
            .and_then(|value| value.as_number())
    }

    pub fn take_console(&mut self) -> Vec<String> {
        take_console_lines()
    }

    fn call_host(&mut self, name: &str, arguments: &[JsValue]) -> Result<JsValue, JsFailure> {
        let global = self.context.global_object();
        let host = global
            .get(js_string!("__frHost"), &mut self.context)
            .map_err(|error| describe_error(error, &mut self.context))?;
        let host_object = host
            .as_object()
            .ok_or_else(|| JsFailure::new("script host is missing"))?;
        let function = host_object
            .get(JsString::from(name), &mut self.context)
            .map_err(|error| describe_error(error, &mut self.context))?;
        let callable = function
            .as_callable()
            .ok_or_else(|| JsFailure::new(format!("script host has no {name}")))?;
        let outcome = callable.call(&host, arguments, &mut self.context);
        self.settle();
        outcome.map_err(|error| describe_error(error, &mut self.context))
    }

    fn settle(&mut self) {
        let _ = self.context.run_jobs();
    }
}
