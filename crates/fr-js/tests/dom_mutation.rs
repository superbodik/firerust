mod support;

use support::Page;

const HTML: &str = "<html><body><ul id=list><li id=a>A</li><li id=b>B</li></ul><div id=target></div></body></html>";

#[test]
fn text_content_can_be_replaced() {
    let mut page = Page::new(HTML);

    page.run("document.getElementById('target').textContent = 'hello'");

    assert_eq!(page.text("target"), "hello");
}

#[test]
fn a_created_element_can_be_appended() {
    let mut page = Page::new(HTML);

    page.run("var li = document.createElement('li'); li.textContent = 'C'; li.id = 'c'; document.getElementById('list').appendChild(li)");

    assert_eq!(page.text("list"), "ABC");
    assert_eq!(page.attribute("c", "id").as_deref(), Some("c"));
}

#[test]
fn a_created_text_node_can_be_appended() {
    let mut page = Page::new(HTML);

    page.run("document.getElementById('target').appendChild(document.createTextNode('plain'))");

    assert_eq!(page.text("target"), "plain");
}

#[test]
fn appending_an_attached_element_moves_it() {
    let mut page = Page::new(HTML);

    page.run("document.getElementById('target').appendChild(document.getElementById('a'))");

    assert_eq!(page.text("list"), "B");
    assert_eq!(page.text("target"), "A");
}

#[test]
fn remove_child_detaches_the_element() {
    let mut page = Page::new(HTML);

    page.run("var list = document.getElementById('list'); list.removeChild(document.getElementById('a'))");

    assert_eq!(page.text("list"), "B");
}

#[test]
fn removing_a_non_child_throws() {
    let mut page = Page::new(HTML);

    let failure = page
        .runtime
        .run_script(
            "document.getElementById('list').removeChild(document.getElementById('target'))",
        )
        .unwrap_err();

    assert!(failure.message().contains("NotFoundError"));
}

#[test]
fn remove_deletes_the_element_itself() {
    let mut page = Page::new(HTML);

    page.run("document.getElementById('b').remove()");

    assert_eq!(page.text("list"), "A");
}

#[test]
fn insert_before_puts_the_element_in_front() {
    let mut page = Page::new(HTML);

    page.run("var li = document.createElement('li'); li.textContent = 'X'; var list = document.getElementById('list'); list.insertBefore(li, document.getElementById('b'))");

    assert_eq!(page.text("list"), "AXB");
}

#[test]
fn insert_before_null_appends() {
    let mut page = Page::new(HTML);

    page.run("var li = document.createElement('li'); li.textContent = 'X'; document.getElementById('list').insertBefore(li, null)");

    assert_eq!(page.text("list"), "ABX");
}

#[test]
fn replace_child_swaps_elements() {
    let mut page = Page::new(HTML);

    page.run("var li = document.createElement('li'); li.textContent = 'Z'; var list = document.getElementById('list'); list.replaceChild(li, document.getElementById('a'))");

    assert_eq!(page.text("list"), "ZB");
}

#[test]
fn append_accepts_several_items_and_text() {
    let mut page = Page::new(HTML);

    page.run("var d = document.getElementById('target'); var s = document.createElement('span'); s.textContent = 'x'; d.append(s, 'y', 'z')");

    assert_eq!(page.text("target"), "xyz");
}

#[test]
fn an_element_cannot_become_its_own_descendant() {
    let mut page = Page::new(HTML);

    let outcome = page
        .runtime
        .run_script("document.getElementById('a').appendChild(document.getElementById('list'))");

    assert!(outcome.is_err());
}

#[test]
fn set_attribute_and_remove_attribute_change_the_document() {
    let mut page = Page::new(HTML);

    page.run("var d = document.getElementById('target'); d.setAttribute('data-x', '5'); d.setAttribute('title', 't')");
    assert_eq!(page.attribute("target", "data-x").as_deref(), Some("5"));

    page.run("document.getElementById('target').removeAttribute('data-x')");
    assert_eq!(page.attribute("target", "data-x"), None);
    assert_eq!(page.attribute("target", "title").as_deref(), Some("t"));
}

#[test]
fn class_list_changes_the_class_attribute() {
    let mut page = Page::new(HTML);

    page.run("var d = document.getElementById('target'); d.classList.add('one', 'two'); d.classList.remove('one'); d.classList.toggle('three'); d.classList.toggle('two', false)");

    assert_eq!(page.attribute("target", "class").as_deref(), Some("three"));
}

#[test]
fn class_list_toggle_reports_the_state() {
    let mut page = Page::new(HTML);

    page.run("var d = document.getElementById('target'); console.log(d.classList.toggle('on'), d.classList.toggle('on'), d.classList.replace('x', 'y'))");

    assert_eq!(page.logs(), vec!["[log] true false false"]);
}

#[test]
fn class_name_can_be_assigned() {
    let mut page = Page::new(HTML);

    page.run("document.getElementById('target').className = 'a b'");

    assert_eq!(page.attribute("target", "class").as_deref(), Some("a b"));
}

#[test]
fn style_properties_write_the_style_attribute() {
    let mut page = Page::new(HTML);

    page.run("var s = document.getElementById('target').style; s.display = 'none'; s.backgroundColor = 'red'");

    assert_eq!(
        page.attribute("target", "style").as_deref(),
        Some("display: none; background-color: red")
    );
}

#[test]
fn style_properties_can_be_read_and_cleared() {
    let mut page = Page::new(HTML);

    page.run("var s = document.getElementById('target').style; s.color = 'blue'; console.log(s.color, s.getPropertyValue('color')); s.removeProperty('color'); console.log(s.color === '', document.getElementById('target').hasAttribute('style'))");

    assert_eq!(page.logs(), vec!["[log] blue blue", "[log] true false"]);
}

#[test]
fn style_css_text_replaces_everything() {
    let mut page = Page::new(HTML);

    page.run("var d = document.getElementById('target'); d.style.color = 'red'; d.style.cssText = 'width: 10px'");

    assert_eq!(
        page.attribute("target", "style").as_deref(),
        Some("width: 10px")
    );
}

#[test]
fn style_set_property_works() {
    let mut page = Page::new(HTML);

    page.run("document.getElementById('target').style.setProperty('margin-top', '4px')");

    assert_eq!(
        page.attribute("target", "style").as_deref(),
        Some("margin-top: 4px")
    );
}

#[test]
fn inner_html_replaces_the_children() {
    let mut page = Page::new(HTML);

    page.run("document.getElementById('target').innerHTML = '<b id=bold>hi</b> there'");

    assert_eq!(page.text("target"), "hi there");
    assert_eq!(page.text("bold"), "hi");
}

#[test]
fn inner_html_can_clear_an_element() {
    let mut page = Page::new(HTML);

    page.run("document.getElementById('list').innerHTML = ''");

    assert_eq!(page.text("list"), "");
}

#[test]
fn clone_node_copies_attributes_and_children() {
    let mut page = Page::new(HTML);

    page.run("var copy = document.getElementById('list').cloneNode(true); document.getElementById('target').appendChild(copy); console.log(copy.id, copy.children.length, copy === document.getElementById('list'))");

    assert_eq!(page.logs(), vec!["[log] list 2 false"]);
    assert_eq!(page.text("target"), "AB");
}

#[test]
fn hidden_and_toggle_attribute_follow_the_attribute() {
    let mut page = Page::new(HTML);

    page.run(
        "var d = document.getElementById('target'); d.hidden = true; d.toggleAttribute('data-on')",
    );

    assert_eq!(page.attribute("target", "hidden").as_deref(), Some(""));
    assert_eq!(page.attribute("target", "data-on").as_deref(), Some(""));
}

#[test]
fn every_change_bumps_the_document_revision() {
    let mut page = Page::new(HTML);
    let before = page.document.revision();

    page.run("document.getElementById('target').textContent = 'x'");

    assert!(page.document.revision() > before);
}

#[test]
fn reading_does_not_bump_the_revision() {
    let mut page = Page::new(HTML);
    let before = page.document.revision();

    page.run("document.getElementById('list').textContent; document.querySelectorAll('li')");

    assert_eq!(page.document.revision(), before);
}
