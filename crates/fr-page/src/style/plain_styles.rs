use fr_css::cascade::compute_styles::compute_styles;
use fr_css::cascade::style_map::StyleMap;
use fr_css::defaults::user_agent_sheet::user_agent_sheet;
use fr_dom::tree::document::Document;

pub fn plain_styles(document: &Document) -> StyleMap {
    compute_styles(document, &user_agent_sheet(), &[])
}
