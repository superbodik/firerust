use fr_css::cascade::style_map::StyleMap;
use fr_dom::node::node_id::NodeId;
use fr_dom::tree::shared_document::SharedDocument;
use fr_image::store::image_store::ImageStore;
use fr_js::runtime::js_runtime::JsRuntime;
use fr_page::load::page::Page;
use fr_page::script::flush_console::flush_console;
use fr_page::style::restyle::restyle;
use url::Url;

use crate::model::layout_cache::LayoutCache;

#[derive(Debug)]
pub struct TabContent {
    pub document: SharedDocument,
    pub styles: StyleMap,
    pub images: ImageStore,
    pub layout: Option<LayoutCache>,
    pub runtime: Option<JsRuntime>,
    seen_revision: u64,
}

impl TabContent {
    pub fn from_page(page: Page) -> Self {
        let seen_revision = page.document.revision();
        Self {
            document: page.document,
            styles: page.styles,
            images: page.images,
            layout: None,
            runtime: page.runtime,
            seen_revision,
        }
    }

    pub fn is_stale(&self) -> bool {
        self.document.revision() != self.seen_revision
    }

    pub fn refresh(&mut self, url: &Url) -> bool {
        if !self.is_stale() {
            return false;
        }
        let (styles, images) = restyle(&self.document.borrow(), url);
        self.styles = styles;
        self.images = images;
        self.layout = None;
        self.seen_revision = self.document.revision();
        true
    }

    pub fn dispatch_event(
        &mut self,
        node: NodeId,
        event_type: &str,
        point: Option<(f32, f32)>,
    ) -> bool {
        let Some(runtime) = self.runtime.as_mut() else {
            return true;
        };
        let allowed = match runtime.dispatch_event(node, event_type, point) {
            Ok(allowed) => allowed,
            Err(error) => {
                eprintln!("script error: {error}");
                true
            }
        };
        flush_console(runtime);
        allowed
    }

    pub fn advance_time(&mut self, now_ms: f64) {
        let Some(runtime) = self.runtime.as_mut() else {
            return;
        };
        if let Err(error) = runtime.advance_time(now_ms) {
            eprintln!("script error: {error}");
        }
        flush_console(runtime);
    }

    pub fn next_timer_due(&mut self) -> Option<f64> {
        self.runtime.as_mut()?.next_timer_due()
    }
}
