mod support;

use support::Page;

const HTML: &str = "<html><body><div id=outer><button id=inner>go</button></div></body></html>";

#[test]
fn a_click_reaches_the_listener() {
    let mut page = Page::new(HTML);
    page.run("document.getElementById('inner').addEventListener('click', function (e) { console.log('clicked', e.type, e.target.id); })");

    let inner = page.id("inner");
    page.runtime.dispatch_event(inner, "click", None).unwrap();

    assert_eq!(page.logs(), vec!["[log] clicked click inner"]);
}

#[test]
fn clicks_bubble_to_ancestors_in_order() {
    let mut page = Page::new(HTML);
    page.run("var log = function (name) { return function (e) { console.log(name, e.currentTarget.id || e.currentTarget.nodeName); }; }; document.getElementById('inner').addEventListener('click', log('inner')); document.getElementById('outer').addEventListener('click', log('outer')); document.addEventListener('click', log('document')); window.addEventListener('click', log('window'));");

    let inner = page.id("inner");
    page.runtime.dispatch_event(inner, "click", None).unwrap();

    assert_eq!(
        page.logs(),
        vec![
            "[log] inner inner",
            "[log] outer outer",
            "[log] document #document",
            "[log] window undefined",
        ]
    );
}

#[test]
fn capture_listeners_run_before_target_listeners() {
    let mut page = Page::new(HTML);
    page.run("document.getElementById('outer').addEventListener('click', function () { console.log('capture'); }, true); document.getElementById('inner').addEventListener('click', function () { console.log('target'); }); document.getElementById('outer').addEventListener('click', function () { console.log('bubble'); });");

    let inner = page.id("inner");
    page.runtime.dispatch_event(inner, "click", None).unwrap();

    assert_eq!(
        page.logs(),
        vec!["[log] capture", "[log] target", "[log] bubble"]
    );
}

#[test]
fn stop_propagation_keeps_ancestors_quiet() {
    let mut page = Page::new(HTML);
    page.run("document.getElementById('inner').addEventListener('click', function (e) { e.stopPropagation(); console.log('inner'); }); document.getElementById('outer').addEventListener('click', function () { console.log('outer'); });");

    let inner = page.id("inner");
    page.runtime.dispatch_event(inner, "click", None).unwrap();

    assert_eq!(page.logs(), vec!["[log] inner"]);
}

#[test]
fn stop_immediate_propagation_skips_other_listeners_on_the_same_target() {
    let mut page = Page::new(HTML);
    page.run("var el = document.getElementById('inner'); el.addEventListener('click', function (e) { e.stopImmediatePropagation(); console.log('first'); }); el.addEventListener('click', function () { console.log('second'); });");

    let inner = page.id("inner");
    page.runtime.dispatch_event(inner, "click", None).unwrap();

    assert_eq!(page.logs(), vec!["[log] first"]);
}

#[test]
fn prevent_default_is_reported_to_the_host() {
    let mut page = Page::new(HTML);
    page.run("document.getElementById('inner').addEventListener('click', function (e) { e.preventDefault(); });");

    let inner = page.id("inner");
    let allowed = page.runtime.dispatch_event(inner, "click", None).unwrap();

    assert!(!allowed);
}

#[test]
fn an_unhandled_click_is_allowed() {
    let mut page = Page::new(HTML);

    let inner = page.id("inner");
    let allowed = page.runtime.dispatch_event(inner, "click", None).unwrap();

    assert!(allowed);
}

#[test]
fn once_listeners_run_a_single_time() {
    let mut page = Page::new(HTML);
    page.run("document.getElementById('inner').addEventListener('click', function () { console.log('once'); }, { once: true });");

    let inner = page.id("inner");
    page.runtime.dispatch_event(inner, "click", None).unwrap();
    page.runtime.dispatch_event(inner, "click", None).unwrap();

    assert_eq!(page.logs(), vec!["[log] once"]);
}

#[test]
fn removed_listeners_stop_firing() {
    let mut page = Page::new(HTML);
    page.run("var handler = function () { console.log('fired'); }; var el = document.getElementById('inner'); el.addEventListener('click', handler); el.removeEventListener('click', handler);");

    let inner = page.id("inner");
    page.runtime.dispatch_event(inner, "click", None).unwrap();

    assert!(page.logs().is_empty());
}

#[test]
fn the_same_listener_is_not_added_twice() {
    let mut page = Page::new(HTML);
    page.run("var handler = function () { console.log('fired'); }; var el = document.getElementById('inner'); el.addEventListener('click', handler); el.addEventListener('click', handler);");

    let inner = page.id("inner");
    page.runtime.dispatch_event(inner, "click", None).unwrap();

    assert_eq!(page.logs(), vec!["[log] fired"]);
}

#[test]
fn onclick_properties_work_and_can_be_replaced() {
    let mut page = Page::new(HTML);
    page.run("var el = document.getElementById('inner'); el.onclick = function () { console.log('one'); }; el.onclick = function () { console.log('two'); };");

    let inner = page.id("inner");
    page.runtime.dispatch_event(inner, "click", None).unwrap();

    assert_eq!(page.logs(), vec!["[log] two"]);
}

#[test]
fn onclick_can_be_cleared() {
    let mut page = Page::new(HTML);
    page.run("var el = document.getElementById('inner'); el.onclick = function () { console.log('one'); }; el.onclick = null;");

    let inner = page.id("inner");
    page.runtime.dispatch_event(inner, "click", None).unwrap();

    assert!(page.logs().is_empty());
}

#[test]
fn an_error_in_a_listener_is_logged_and_the_others_still_run() {
    let mut page = Page::new(HTML);
    page.run("var el = document.getElementById('inner'); el.addEventListener('click', function () { throw new Error('bad'); }); el.addEventListener('click', function () { console.log('next'); });");

    let inner = page.id("inner");
    page.runtime.dispatch_event(inner, "click", None).unwrap();

    let logs = page.logs();
    assert!(logs[0].starts_with("[error] Uncaught Error: bad"));
    assert_eq!(logs[1], "[log] next");
}

#[test]
fn click_events_carry_the_pointer_position() {
    let mut page = Page::new(HTML);
    page.run("document.getElementById('inner').addEventListener('click', function (e) { console.log(e.clientX, e.clientY); });");

    let inner = page.id("inner");
    page.runtime
        .dispatch_event(inner, "click", Some((12.0, 34.0)))
        .unwrap();

    assert_eq!(page.logs(), vec!["[log] 12 34"]);
}

#[test]
fn the_click_method_dispatches_a_click() {
    let mut page = Page::new(HTML);
    page.run("document.getElementById('inner').addEventListener('click', function () { console.log('via click()'); }); document.getElementById('inner').click();");

    assert_eq!(page.logs(), vec!["[log] via click()"]);
}

#[test]
fn custom_events_carry_detail() {
    let mut page = Page::new(HTML);
    page.run("var el = document.getElementById('inner'); el.addEventListener('ping', function (e) { console.log(e.detail.n); }); el.dispatchEvent(new CustomEvent('ping', { detail: { n: 9 } }));");

    assert_eq!(page.logs(), vec!["[log] 9"]);
}

#[test]
fn custom_events_do_not_bubble_by_default() {
    let mut page = Page::new(HTML);
    page.run("document.getElementById('outer').addEventListener('ping', function () { console.log('outer'); }); document.getElementById('inner').dispatchEvent(new Event('ping')); document.getElementById('inner').dispatchEvent(new Event('ping', { bubbles: true }));");

    assert_eq!(page.logs(), vec!["[log] outer"]);
}

#[test]
fn keyboard_events_carry_the_key() {
    let mut page = Page::new(HTML);
    page.run("document.addEventListener('keydown', function (e) { console.log(e.key, e instanceof KeyboardEvent); });");

    let inner = page.id("inner");
    page.runtime.dispatch_event(inner, "keydown", None).unwrap();

    assert_eq!(page.logs(), vec!["[log]  true"]);
}

#[test]
fn dom_content_loaded_fires_on_the_document() {
    let mut page = Page::new(HTML);
    page.run("console.log(document.readyState); document.addEventListener('DOMContentLoaded', function () { console.log('ready', document.readyState); });");

    page.runtime.fire_event("DOMContentLoaded").unwrap();

    assert_eq!(
        page.logs(),
        vec!["[log] loading", "[log] ready interactive"]
    );
}

#[test]
fn load_fires_on_the_window() {
    let mut page = Page::new(HTML);
    page.run("window.addEventListener('load', function () { console.log('loaded', document.readyState); }); window.onload = function () { console.log('onload'); };");

    page.runtime.fire_event("load").unwrap();

    assert_eq!(page.logs(), vec!["[log] loaded complete", "[log] onload"]);
}

#[test]
fn a_handler_can_change_the_document() {
    let mut page = Page::new(HTML);
    page.run("var count = 0; document.getElementById('inner').addEventListener('click', function () { count += 1; document.getElementById('outer').textContent = 'clicked ' + count; });");

    let inner = page.id("inner");
    page.runtime.dispatch_event(inner, "click", None).unwrap();

    assert_eq!(page.text("outer"), "clicked 1");
}
