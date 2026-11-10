use fr_page::load::page::Page;

use crate::model::tab::Tab;
use crate::model::tab_id::TabId;

#[derive(Debug)]
pub struct TabSet {
    tabs: Vec<Tab>,
    active: usize,
    next_id: u32,
    clock: u64,
}

impl TabSet {
    pub fn new(first: Page) -> Self {
        Self {
            tabs: vec![Tab::from_page(TabId(0), first, 0)],
            active: 0,
            next_id: 1,
            clock: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.tabs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }

    pub fn tabs(&self) -> &[Tab] {
        &self.tabs
    }

    pub fn active_index(&self) -> usize {
        self.active
    }

    pub fn active(&self) -> &Tab {
        &self.tabs[self.active]
    }

    pub fn active_mut(&mut self) -> &mut Tab {
        &mut self.tabs[self.active]
    }

    pub fn tab_mut(&mut self, index: usize) -> Option<&mut Tab> {
        self.tabs.get_mut(index)
    }

    pub fn open(&mut self, page: Page) -> usize {
        self.clock += 1;
        let id = TabId(self.next_id);
        self.next_id += 1;
        self.tabs.push(Tab::from_page(id, page, self.clock));
        self.active = self.tabs.len() - 1;
        self.active
    }

    pub fn activate(&mut self, index: usize) -> bool {
        if index >= self.tabs.len() || index == self.active {
            return false;
        }
        self.clock += 1;
        self.active = index;
        self.tabs[index].touch(self.clock);
        true
    }

    pub fn close(&mut self, index: usize) -> bool {
        if self.tabs.len() <= 1 || index >= self.tabs.len() {
            return false;
        }
        self.tabs.remove(index);
        if index < self.active {
            self.active -= 1;
        } else if index == self.active {
            self.active = index.min(self.tabs.len() - 1);
            self.clock += 1;
            self.tabs[self.active].touch(self.clock);
        }
        true
    }

    pub fn enforce_limit(&mut self, max_live: usize) -> usize {
        let keep_background = max_live.saturating_sub(1);
        let mut background_live: Vec<usize> = (0..self.tabs.len())
            .filter(|&index| index != self.active && !self.tabs[index].is_discarded())
            .collect();
        background_live.sort_by_key(|&index| std::cmp::Reverse(self.tabs[index].last_used()));
        let mut discarded = 0;
        for &index in background_live.iter().skip(keep_background) {
            self.tabs[index].discard();
            discarded += 1;
        }
        discarded
    }
}
