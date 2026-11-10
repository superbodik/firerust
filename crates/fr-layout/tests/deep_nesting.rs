mod support;

use support::layout;

fn nested(depth: usize) -> String {
    format!(
        "{}<p id=leaf>deep</p>{}",
        "<div>".repeat(depth),
        "</div>".repeat(depth)
    )
}

#[test]
fn a_thousand_nested_blocks_are_laid_out() {
    let page = layout(&nested(1_000), "", 800.0);

    assert!(page.find("leaf").is_some());
}

#[test]
fn five_thousand_nested_blocks_are_laid_out() {
    let page = layout(&nested(5_000), "", 800.0);

    assert!(page.find("leaf").is_some());
}
