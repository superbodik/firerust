use url::Url;

use crate::history::history_entry::HistoryEntry;

#[derive(Debug, Clone, Default)]
pub struct NavigationHistory {
    entries: Vec<HistoryEntry>,
    position: usize,
}

impl NavigationHistory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn current(&self) -> Option<&HistoryEntry> {
        self.entries.get(self.position)
    }

    pub fn entries(&self) -> &[HistoryEntry] {
        &self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn can_go_back(&self) -> bool {
        self.position > 0
    }

    pub fn can_go_forward(&self) -> bool {
        self.position + 1 < self.entries.len()
    }

    pub fn visit(&mut self, url: Url) {
        if !self.entries.is_empty() {
            self.entries.truncate(self.position + 1);
            self.position += 1;
        }
        self.entries.push(HistoryEntry::new(url));
    }

    pub fn replace_current(&mut self, url: Url) {
        match self.entries.get_mut(self.position) {
            Some(entry) => *entry = HistoryEntry::new(url),
            None => self.entries.push(HistoryEntry::new(url)),
        }
    }

    pub fn remember_scroll(&mut self, scroll: f32) {
        if let Some(entry) = self.entries.get_mut(self.position) {
            entry.scroll = scroll;
        }
    }

    pub fn go_back(&mut self) -> Option<&HistoryEntry> {
        if !self.can_go_back() {
            return None;
        }
        self.position -= 1;
        self.current()
    }

    pub fn go_forward(&mut self) -> Option<&HistoryEntry> {
        if !self.can_go_forward() {
            return None;
        }
        self.position += 1;
        self.current()
    }
}
