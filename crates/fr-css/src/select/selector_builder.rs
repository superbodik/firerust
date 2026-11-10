use cssparser::Token;

use crate::select::attribute_condition::AttributeCondition;
use crate::select::combinator::Combinator;
use crate::select::compound::Compound;
use crate::select::selector::Selector;
use crate::select::selector_part::SelectorPart;

pub struct SelectorBuilder {
    parts: Vec<SelectorPart>,
    compound: Compound,
    has_content: bool,
    combinator: Combinator,
    is_expecting_compound: bool,
    is_after_dot: bool,
    is_valid: bool,
}

impl SelectorBuilder {
    pub fn new() -> Self {
        Self {
            parts: Vec::new(),
            compound: Compound::default(),
            has_content: false,
            combinator: Combinator::Descendant,
            is_expecting_compound: false,
            is_after_dot: false,
            is_valid: true,
        }
    }

    pub fn accept(&mut self, token: &Token<'_>) {
        match token {
            Token::WhiteSpace(_) => self.close_compound(),
            Token::Delim('>') => {
                self.close_compound();
                if self.parts.is_empty() {
                    self.is_valid = false;
                }
                self.combinator = Combinator::Child;
                self.is_expecting_compound = true;
            }
            Token::Delim('.') => self.is_after_dot = true,
            Token::Delim('*') => self.touch(),
            Token::Ident(name) => {
                let text: &str = name;
                self.accept_name(text);
            }
            Token::IDHash(name) => {
                if self.compound.id.is_some() {
                    self.is_valid = false;
                }
                let text: &str = name;
                self.compound.id = Some(text.into());
                self.touch();
            }
            _ => self.is_valid = false,
        }
    }

    pub fn accept_attribute(&mut self, condition: AttributeCondition) {
        self.compound.attributes.push(condition);
        self.touch();
    }

    pub fn invalidate(&mut self) {
        self.is_valid = false;
    }

    pub fn finish(&mut self) -> Option<Selector> {
        self.close_compound();
        let is_complete = self.is_valid
            && !self.is_expecting_compound
            && !self.is_after_dot
            && !self.parts.is_empty();
        let parts = std::mem::take(&mut self.parts);
        *self = Self::new();
        if is_complete {
            Some(Selector { parts })
        } else {
            None
        }
    }

    fn accept_name(&mut self, name: &str) {
        if self.is_after_dot {
            self.compound.classes.push(name.into());
            self.is_after_dot = false;
        } else if self.has_content {
            self.is_valid = false;
        } else {
            self.compound.tag = Some(name.to_ascii_lowercase().into());
        }
        self.touch();
    }

    fn touch(&mut self) {
        self.has_content = true;
        self.is_expecting_compound = false;
    }

    fn close_compound(&mut self) {
        if !self.has_content {
            return;
        }
        let compound = std::mem::take(&mut self.compound);
        self.parts.push(SelectorPart {
            combinator: self.combinator,
            compound,
        });
        self.has_content = false;
        self.combinator = Combinator::Descendant;
    }
}

impl Default for SelectorBuilder {
    fn default() -> Self {
        Self::new()
    }
}
