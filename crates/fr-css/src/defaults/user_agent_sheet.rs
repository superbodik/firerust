use crate::defaults::user_agent_css::USER_AGENT_CSS;
use crate::parse::parse_stylesheet::parse_stylesheet;
use crate::parse::stylesheet::Stylesheet;

pub fn user_agent_sheet() -> Stylesheet {
    parse_stylesheet(USER_AGENT_CSS)
}
