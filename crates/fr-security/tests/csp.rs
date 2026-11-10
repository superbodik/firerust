use fr_html::parse::parse_html::parse_html;
use fr_security::csp::parse_policy::parse_policy;
use fr_security::fetch::block_reason::BlockReason;
use fr_security::fetch::resource_kind::ResourceKind;
use fr_security::guard::page_guard::PageGuard;
use url::Url;

fn url(text: &str) -> Url {
    Url::parse(text).unwrap()
}

fn guard(policy: &str) -> PageGuard {
    let html = format!(
        "<html><head><meta http-equiv=\"Content-Security-Policy\" content=\"{policy}\"></head><body></body></html>"
    );
    PageGuard::new(&parse_html(&html), &url("https://a.test/page"))
}

#[test]
fn a_page_without_a_policy_allows_everything_that_passes_the_basic_checks() {
    let guard = PageGuard::new(&parse_html("<p>x</p>"), &url("https://a.test/"));

    assert!(
        guard
            .check(ResourceKind::Script, &url("https://cdn.test/a.js"))
            .is_ok()
    );
    assert!(guard.allows_inline(ResourceKind::Script));
    assert!(guard.allows_inline(ResourceKind::Style));
}

#[test]
fn self_allows_only_the_same_origin() {
    let guard = guard("script-src 'self'");

    assert!(
        guard
            .check(ResourceKind::Script, &url("https://a.test/a.js"))
            .is_ok()
    );
    assert_eq!(
        guard.check(ResourceKind::Script, &url("https://cdn.test/a.js")),
        Err(BlockReason::ContentPolicy)
    );
}

#[test]
fn none_blocks_everything_of_that_kind() {
    let guard = guard("img-src 'none'");

    assert_eq!(
        guard.check(ResourceKind::Image, &url("https://a.test/x.png")),
        Err(BlockReason::ContentPolicy)
    );
    assert!(
        guard
            .check(ResourceKind::Script, &url("https://a.test/a.js"))
            .is_ok()
    );
}

#[test]
fn default_src_covers_kinds_without_their_own_directive() {
    let guard = guard("default-src 'self'; img-src *");

    assert!(
        guard
            .check(ResourceKind::Image, &url("https://cdn.test/x.png"))
            .is_ok()
    );
    assert_eq!(
        guard.check(ResourceKind::Style, &url("https://cdn.test/a.css")),
        Err(BlockReason::ContentPolicy)
    );
    assert!(
        guard
            .check(ResourceKind::Style, &url("https://a.test/a.css"))
            .is_ok()
    );
}

#[test]
fn host_sources_match_names_wildcards_schemes_and_ports() {
    let guard = guard("script-src cdn.test *.static.test https://secure.test:8443 data:");

    assert!(
        guard
            .check(ResourceKind::Script, &url("https://cdn.test/a.js"))
            .is_ok()
    );
    assert!(
        guard
            .check(ResourceKind::Script, &url("https://x.static.test/a.js"))
            .is_ok()
    );
    assert!(
        guard
            .check(ResourceKind::Script, &url("https://secure.test:8443/a.js"))
            .is_ok()
    );
    assert!(
        guard
            .check(ResourceKind::Script, &url("https://secure.test/a.js"))
            .is_err()
    );
    assert!(
        guard
            .check(ResourceKind::Script, &url("https://static.test/a.js"))
            .is_err()
    );
    assert!(
        guard
            .check(ResourceKind::Script, &url("https://evil.test/a.js"))
            .is_err()
    );
}

#[test]
fn inline_content_needs_unsafe_inline() {
    assert!(!guard("script-src 'self'").allows_inline(ResourceKind::Script));
    assert!(guard("script-src 'self'").allows_inline(ResourceKind::Style));
    assert!(guard("script-src 'self' 'unsafe-inline'").allows_inline(ResourceKind::Script));
    assert!(!guard("default-src 'self'").allows_inline(ResourceKind::Style));
}

#[test]
fn several_policies_must_all_agree() {
    let html = "<meta http-equiv=content-security-policy content=\"img-src *\">\
                <meta http-equiv=content-security-policy content=\"img-src https://ok.test\">";
    let guard = PageGuard::new(&parse_html(html), &url("https://a.test/"));

    assert!(
        guard
            .check(ResourceKind::Image, &url("https://ok.test/x.png"))
            .is_ok()
    );
    assert!(
        guard
            .check(ResourceKind::Image, &url("https://other.test/x.png"))
            .is_err()
    );
}

#[test]
fn the_first_copy_of_a_directive_wins_and_unknown_ones_are_ignored() {
    let policy = parse_policy("img-src 'none'; img-src *; frame-ancestors 'none'; ;");

    assert_eq!(policy.img_src.map(|list| list.items.len()), Some(0));
    assert!(policy.default_src.is_none());
}

#[test]
fn the_basic_checks_still_apply_under_a_policy() {
    let guard = guard("img-src *");

    assert_eq!(
        guard.check(ResourceKind::Image, &url("http://insecure.test/x.png")),
        Err(BlockReason::MixedContent)
    );
}
