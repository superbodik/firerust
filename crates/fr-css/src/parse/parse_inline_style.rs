use cssparser::Parser;

use crate::parse::parse_declaration_block::parse_declaration_block;
use crate::parse::style_declaration::StyleDeclaration;

pub fn parse_inline_style(css: &str) -> Vec<StyleDeclaration> {
    let mut parser = Parser::new(css);
    parse_declaration_block(&mut parser)
}
