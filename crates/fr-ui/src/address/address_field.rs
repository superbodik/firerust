#[derive(Debug, Clone, Default, PartialEq)]
pub struct AddressField {
    text: String,
    caret: usize,
    is_focused: bool,
    selects_all: bool,
}

impl AddressField {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn caret(&self) -> usize {
        self.caret
    }

    pub fn is_focused(&self) -> bool {
        self.is_focused
    }

    pub fn selects_all(&self) -> bool {
        self.selects_all
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_owned();
        self.caret = self.length();
        self.selects_all = false;
    }

    pub fn focus(&mut self) {
        self.is_focused = true;
        self.selects_all = true;
        self.caret = self.length();
    }

    pub fn blur(&mut self) {
        self.is_focused = false;
        self.selects_all = false;
    }

    pub fn insert(&mut self, character: char) {
        self.clear_selection();
        let at = self.byte_index(self.caret);
        self.text.insert(at, character);
        self.caret += 1;
    }

    pub fn backspace(&mut self) {
        if self.clear_selection() || self.caret == 0 {
            return;
        }
        let at = self.byte_index(self.caret - 1);
        self.text.remove(at);
        self.caret -= 1;
    }

    pub fn delete(&mut self) {
        if self.clear_selection() || self.caret >= self.length() {
            return;
        }
        let at = self.byte_index(self.caret);
        self.text.remove(at);
    }

    pub fn move_left(&mut self) {
        self.selects_all = false;
        self.caret = self.caret.saturating_sub(1);
    }

    pub fn move_right(&mut self) {
        self.selects_all = false;
        self.caret = (self.caret + 1).min(self.length());
    }

    pub fn move_home(&mut self) {
        self.selects_all = false;
        self.caret = 0;
    }

    pub fn move_end(&mut self) {
        self.selects_all = false;
        self.caret = self.length();
    }

    fn clear_selection(&mut self) -> bool {
        if !self.selects_all {
            return false;
        }
        self.text.clear();
        self.caret = 0;
        self.selects_all = false;
        true
    }

    fn length(&self) -> usize {
        self.text.chars().count()
    }

    fn byte_index(&self, caret: usize) -> usize {
        self.text
            .char_indices()
            .nth(caret)
            .map_or(self.text.len(), |(index, _)| index)
    }
}
