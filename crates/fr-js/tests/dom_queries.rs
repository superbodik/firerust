mod support;

use support::Page;

const HTML: &str = "<html><head><title>Demo</title></head><body>\
<div id=box class='card wide'><p id=first>one</p><p id=second class=note>two</p><span>three</span></div>\
<a id=link href='/next'>go</a></body></html>";

#[test]
fn get_element_by_id_finds_the_element() {
    let mut page = Page::new(HTML);

    page.run(
        "var el = document.getElementById('first'); console.log(el.tagName, el.id, el.textContent)",
    );

    assert_eq!(page.logs(), vec!["[log] P first one"]);
}

#[test]
fn a_missing_id_gives_null() {
    let mut page = Page::new(HTML);

    page.run("console.log(document.getElementById('nope'))");

    assert_eq!(page.logs(), vec!["[log] null"]);
}

#[test]
fn the_same_element_is_the_same_object() {
    let mut page = Page::new(HTML);

    page.run("console.log(document.getElementById('box') === document.querySelector('#box'))");

    assert_eq!(page.logs(), vec!["[log] true"]);
}

#[test]
fn query_selector_understands_class_and_tag_and_descendant() {
    let mut page = Page::new(HTML);

    page.run("console.log(document.querySelector('.note').id, document.querySelector('div > span').textContent, document.querySelector('body p').id)");

    assert_eq!(page.logs(), vec!["[log] second three first"]);
}

#[test]
fn query_selector_all_returns_every_match_in_order() {
    let mut page = Page::new(HTML);

    page.run(
        "console.log(document.querySelectorAll('p').map(function (p) { return p.id; }).join(','))",
    );

    assert_eq!(page.logs(), vec!["[log] first,second"]);
}

#[test]
fn scoped_queries_stay_inside_the_element() {
    let mut page = Page::new(HTML);

    page.run("var box = document.getElementById('box'); console.log(box.querySelectorAll('*').length, box.querySelector('a'))");

    assert_eq!(page.logs(), vec!["[log] 3 null"]);
}

#[test]
fn document_exposes_its_structure() {
    let mut page = Page::new(HTML);

    page.run("console.log(document.documentElement.tagName, document.body.tagName, document.head.tagName, document.title)");

    assert_eq!(page.logs(), vec!["[log] HTML BODY HEAD Demo"]);
}

#[test]
fn tree_navigation_works() {
    let mut page = Page::new(HTML);

    page.run("var p = document.getElementById('first'); console.log(p.parentNode.id, p.nextElementSibling.id, p.nextSibling.id, p.previousSibling, p.parentElement.children.length)");

    assert_eq!(page.logs(), vec!["[log] box second second null 3"]);
}

#[test]
fn attributes_can_be_read() {
    let mut page = Page::new(HTML);

    page.run("var a = document.getElementById('link'); console.log(a.getAttribute('href'), a.hasAttribute('href'), a.getAttribute('rel'), a.href)");

    assert_eq!(page.logs(), vec!["[log] /next true null /next"]);
}

#[test]
fn class_list_reads_classes() {
    let mut page = Page::new(HTML);

    page.run("var c = document.getElementById('box'); console.log(c.className, c.classList.contains('wide'), c.classList.contains('x'), c.classList.length)");

    assert_eq!(page.logs(), vec!["[log] card wide true false 2"]);
}

#[test]
fn matches_and_closest_use_selectors() {
    let mut page = Page::new(HTML);

    page.run("var p = document.getElementById('second'); console.log(p.matches('.note'), p.matches('span'), p.closest('div').id, p.closest('.missing'))");

    assert_eq!(page.logs(), vec!["[log] true false box null"]);
}

#[test]
fn elements_by_tag_and_class_name_are_found() {
    let mut page = Page::new(HTML);

    page.run("console.log(document.getElementsByTagName('p').length, document.getElementsByClassName('card wide').length)");

    assert_eq!(page.logs(), vec!["[log] 2 1"]);
}

#[test]
fn node_types_and_names_are_reported() {
    let mut page = Page::new(HTML);

    page.run("var p = document.getElementById('first'); console.log(p.nodeType, p.nodeName, p.firstChild.nodeType, p.firstChild.nodeName, document.nodeType)");

    assert_eq!(page.logs(), vec!["[log] 1 P 3 #text 9"]);
}

#[test]
fn inner_html_serializes_the_children() {
    let mut page = Page::new(HTML);

    page.run("console.log(document.getElementById('second').parentNode.innerHTML)");

    assert_eq!(
        page.logs(),
        vec![r#"[log] <p id="first">one</p><p id="second" class="note">two</p><span>three</span>"#]
    );
}
