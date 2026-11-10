pub struct LineBuilder {
    lines: Vec<String>,
    current: String,
    has_pending_space: bool,
}

impl LineBuilder {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            current: String::new(),
            has_pending_space: false,
        }
    }

    pub fn push_text(&mut self, text: &str) {
        for character in text.chars() {
            if character.is_whitespace() && character != '\u{a0}' {
                self.has_pending_space = true;
                continue;
            }
            if self.has_pending_space && !self.current.is_empty() {
                self.current.push(' ');
            }
            self.has_pending_space = false;
            self.current.push(character);
        }
    }

    pub fn start_bullet(&mut self) {
        self.break_line();
        self.current.push('\u{2022}');
        self.has_pending_space = true;
    }

    pub fn break_line(&mut self) {
        if !self.current.is_empty() {
            self.lines.push(std::mem::take(&mut self.current));
        }
        self.has_pending_space = false;
    }

    pub fn gap(&mut self) {
        self.break_line();
        if self.lines.last().is_some_and(|line| !line.is_empty()) {
            self.lines.push(String::new());
        }
    }

    pub fn finish(mut self) -> Vec<String> {
        self.break_line();
        while self.lines.last().is_some_and(String::is_empty) {
            self.lines.pop();
        }
        self.lines
    }
}

impl Default for LineBuilder {
    fn default() -> Self {
        Self::new()
    }
}
