pub struct PathReader<'a> {
    text: &'a [u8],
    position: usize,
}

impl<'a> PathReader<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text: text.as_bytes(),
            position: 0,
        }
    }

    fn skip_separators(&mut self) {
        while self
            .text
            .get(self.position)
            .is_some_and(|byte| byte.is_ascii_whitespace() || *byte == b',')
        {
            self.position += 1;
        }
    }

    pub fn is_finished(&mut self) -> bool {
        self.skip_separators();
        self.position >= self.text.len()
    }

    pub fn next_command(&mut self) -> Option<char> {
        self.skip_separators();
        let byte = *self.text.get(self.position)?;
        if byte.is_ascii_alphabetic() {
            self.position += 1;
            return Some(char::from(byte));
        }
        None
    }

    pub fn has_number(&mut self) -> bool {
        self.skip_separators();
        self.text
            .get(self.position)
            .is_some_and(|byte| byte.is_ascii_digit() || matches!(byte, b'-' | b'+' | b'.'))
    }

    pub fn number(&mut self) -> Option<f32> {
        self.skip_separators();
        let start = self.position;
        if matches!(self.text.get(self.position), Some(b'-' | b'+')) {
            self.position += 1;
        }
        self.digits();
        if self.text.get(self.position) == Some(&b'.') {
            self.position += 1;
            self.digits();
        }
        if matches!(self.text.get(self.position), Some(b'e' | b'E')) {
            let mark = self.position;
            self.position += 1;
            if matches!(self.text.get(self.position), Some(b'-' | b'+')) {
                self.position += 1;
            }
            if self.text.get(self.position).is_some_and(u8::is_ascii_digit) {
                self.digits();
            } else {
                self.position = mark;
            }
        }
        let slice = std::str::from_utf8(&self.text[start..self.position]).ok()?;
        slice.parse::<f32>().ok()
    }

    pub fn flag(&mut self) -> Option<bool> {
        self.skip_separators();
        let byte = *self.text.get(self.position)?;
        self.position += 1;
        match byte {
            b'0' => Some(false),
            b'1' => Some(true),
            _ => None,
        }
    }

    fn digits(&mut self) {
        while self.text.get(self.position).is_some_and(u8::is_ascii_digit) {
            self.position += 1;
        }
    }
}
