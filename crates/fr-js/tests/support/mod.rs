#![allow(dead_code)]

use fr_dom::node::node_id::NodeId;
use fr_dom::query::text_content::text_content;
use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;
use fr_dom::tree::shared_document::SharedDocument;
use fr_html::parse::parse_html::parse_html;
use fr_js::runtime::js_runtime::JsRuntime;

pub struct Page {
    pub runtime: JsRuntime,
    pub document: SharedDocument,
}

impl Page {
    pub fn new(html: &str) -> Self {
        let document = SharedDocument::new(parse_html(html));
        let runtime = JsRuntime::new(document.clone(), "https://example.test/page").unwrap();
        Self { runtime, document }
    }

    pub fn run(&mut self, script: &str) {
        self.runtime.run_script(script).unwrap();
    }

    pub fn logs(&mut self) -> Vec<String> {
        self.runtime.take_console()
    }

    pub fn id(&self, wanted: &str) -> NodeId {
        find_by_id(&self.document.borrow(), wanted)
    }

    pub fn text(&self, wanted: &str) -> String {
        let id = self.id(wanted);
        text_content(&self.document.borrow(), id)
    }

    pub fn attribute(&self, wanted: &str, name: &str) -> Option<String> {
        let id = self.id(wanted);
        self.document
            .borrow()
            .node(id)
            .and_then(|node| node.kind().as_element())
            .and_then(|element| element.attribute(name).map(str::to_owned))
    }
}

pub fn find_by_id(document: &Document, wanted: &str) -> NodeId {
    document
        .walk(document.root())
        .find_map(|edge| match edge {
            Edge::Enter(id) => {
                let element = document.node(id)?.kind().as_element()?;
                (element.attribute("id") == Some(wanted)).then_some(id)
            }
            Edge::Leave(_) => None,
        })
        .expect("element with the id")
}
