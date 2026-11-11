use fr_history::history::navigation_history::NavigationHistory;
use fr_page::load::page::Page;
use url::Url;

use crate::footprint::estimate_footprint::estimate_footprint;
use crate::model::tab_content::TabContent;
use crate::model::tab_id::TabId;

#[derive(Debug)]
pub struct Tab {
    id: TabId,
    url: Url,
    title: Option<String>,
    scroll: f32,
    history: NavigationHistory,
    content: Option<TabContent>,
    reader_original: Option<TabContent>,
    last_used: u64,
}

impl Tab {
    pub fn from_page(id: TabId, page: Page, last_used: u64) -> Self {
        let mut history = NavigationHistory::new();
        history.visit(page.url.clone());
        Self {
            id,
            url: page.url.clone(),
            title: page.title.clone(),
            scroll: 0.0,
            history,
            content: Some(TabContent::from_page(page)),
            reader_original: None,
            last_used,
        }
    }

    pub fn id(&self) -> TabId {
        self.id
    }

    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn title(&self) -> String {
        match &self.title {
            Some(title) if !title.trim().is_empty() => title.clone(),
            _ => self.url.to_string(),
        }
    }

    pub fn scroll(&self) -> f32 {
        self.scroll
    }

    pub fn set_scroll(&mut self, scroll: f32) {
        self.scroll = scroll;
    }

    pub fn last_used(&self) -> u64 {
        self.last_used
    }

    pub fn touch(&mut self, clock: u64) {
        self.last_used = clock;
    }

    pub fn history(&self) -> &NavigationHistory {
        &self.history
    }

    pub fn history_mut(&mut self) -> &mut NavigationHistory {
        &mut self.history
    }

    pub fn content(&self) -> Option<&TabContent> {
        self.content.as_ref()
    }

    pub fn content_mut(&mut self) -> Option<&mut TabContent> {
        self.content.as_mut()
    }

    pub fn is_discarded(&self) -> bool {
        self.content.is_none()
    }

    pub fn discard(&mut self) {
        self.content = None;
        self.reader_original = None;
    }

    pub fn is_reading(&self) -> bool {
        self.reader_original.is_some()
    }

    pub fn enter_reader(&mut self, page: Page) -> bool {
        let Some(original) = self.content.take() else {
            return false;
        };
        self.reader_original = Some(original);
        self.title = page.title.clone();
        self.content = Some(TabContent::from_page(page));
        true
    }

    pub fn leave_reader(&mut self) -> bool {
        let Some(original) = self.reader_original.take() else {
            return false;
        };
        self.content = Some(original);
        true
    }

    pub fn show(&mut self, page: Page) {
        self.reader_original = None;
        self.url = page.url.clone();
        self.title = page.title.clone();
        self.content = Some(TabContent::from_page(page));
    }

    pub fn set_url(&mut self, url: Url) {
        self.url = url;
    }

    pub fn restore(&mut self, page: Page) {
        self.reader_original = None;
        self.title = page.title.clone();
        self.content = Some(TabContent::from_page(page));
    }

    pub fn footprint_bytes(&self) -> usize {
        self.content.as_ref().map_or(0, estimate_footprint)
    }
}
