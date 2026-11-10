use fr_dom::error::dom_error::DomError;
use fr_dom::node::attribute::Attribute;
use fr_dom::node::node_id::NodeId;
use fr_dom::query::text_content::text_content;
use fr_dom::tree::document::Document;

fn element(document: &mut Document, name: &str) -> NodeId {
    document.create_element(name, Vec::new()).unwrap()
}

fn names(document: &Document, parent: NodeId) -> Vec<String> {
    document
        .children(parent)
        .map(|id| {
            document
                .node(id)
                .unwrap()
                .kind()
                .as_element()
                .map_or_else(String::new, |element| element.name().to_owned())
        })
        .collect()
}

fn list_of_three() -> (Document, NodeId, NodeId, NodeId, NodeId) {
    let mut document = Document::new();
    let list = element(&mut document, "ul");
    let first = element(&mut document, "a");
    let second = element(&mut document, "b");
    let third = element(&mut document, "c");
    document.append_child(document.root(), list).unwrap();
    for child in [first, second, third] {
        document.append_child(list, child).unwrap();
    }
    (document, list, first, second, third)
}

#[test]
fn the_revision_grows_with_every_change() {
    let mut document = Document::new();
    let before = document.revision();
    let node = element(&mut document, "p");

    document.append_child(document.root(), node).unwrap();

    assert!(document.revision() > before);
}

#[test]
fn setting_an_attribute_adds_and_replaces() {
    let mut document = Document::new();
    let node = document
        .create_element("p", vec![Attribute::new("class", "a")])
        .unwrap();

    document.set_attribute(node, "class", "b").unwrap();
    document.set_attribute(node, "id", "x").unwrap();

    let element = document.node(node).unwrap().kind().as_element().unwrap();
    assert_eq!(element.attribute("class"), Some("b"));
    assert_eq!(element.attribute("id"), Some("x"));
    assert_eq!(element.attributes().len(), 2);
}

#[test]
fn removing_an_attribute_removes_it() {
    let mut document = Document::new();
    let node = document
        .create_element("p", vec![Attribute::new("class", "a")])
        .unwrap();

    document.remove_attribute(node, "class").unwrap();

    let element = document.node(node).unwrap().kind().as_element().unwrap();
    assert_eq!(element.attribute("class"), None);
}

#[test]
fn removing_a_missing_attribute_keeps_the_revision() {
    let mut document = Document::new();
    let node = element(&mut document, "p");
    let before = document.revision();

    document.remove_attribute(node, "class").unwrap();

    assert_eq!(document.revision(), before);
}

#[test]
fn attributes_belong_to_elements_only() {
    let mut document = Document::new();
    let text = document.create_text("hi").unwrap();

    assert_eq!(
        document.set_attribute(text, "class", "a"),
        Err(DomError::NotAnElement)
    );
}

#[test]
fn detaching_the_middle_child_joins_its_neighbours() {
    let (mut document, list, first, second, third) = list_of_three();

    document.detach(second).unwrap();

    assert_eq!(names(&document, list), vec!["a", "c"]);
    assert_eq!(document.node(first).unwrap().next_sibling(), Some(third));
    assert_eq!(document.node(third).unwrap().prev_sibling(), Some(first));
    assert_eq!(document.node(second).unwrap().parent(), None);
}

#[test]
fn detaching_the_first_and_last_child_updates_the_parent() {
    let (mut document, list, first, _second, third) = list_of_three();

    document.detach(first).unwrap();
    document.detach(third).unwrap();

    assert_eq!(names(&document, list), vec!["b"]);
    let parent = document.node(list).unwrap();
    assert_eq!(parent.first_child(), parent.last_child());
}

#[test]
fn a_detached_node_can_be_attached_again() {
    let (mut document, list, first, _second, _third) = list_of_three();

    document.detach(first).unwrap();
    document.append_child(list, first).unwrap();

    assert_eq!(names(&document, list), vec!["b", "c", "a"]);
}

#[test]
fn detaching_a_node_without_a_parent_is_harmless() {
    let mut document = Document::new();
    let node = element(&mut document, "p");

    assert_eq!(document.detach(node), Ok(()));
}

#[test]
fn inserting_before_a_reference_places_the_child_in_front_of_it() {
    let (mut document, list, _first, second, _third) = list_of_three();
    let extra = element(&mut document, "x");

    document.insert_before(list, extra, Some(second)).unwrap();

    assert_eq!(names(&document, list), vec!["a", "x", "b", "c"]);
}

#[test]
fn inserting_before_the_first_child_updates_the_parent() {
    let (mut document, list, first, _second, _third) = list_of_three();
    let extra = element(&mut document, "x");

    document.insert_before(list, extra, Some(first)).unwrap();

    assert_eq!(names(&document, list), vec!["x", "a", "b", "c"]);
    assert_eq!(document.node(list).unwrap().first_child(), Some(extra));
}

#[test]
fn inserting_without_a_reference_appends() {
    let (mut document, list, _first, _second, _third) = list_of_three();
    let extra = element(&mut document, "x");

    document.insert_before(list, extra, None).unwrap();

    assert_eq!(names(&document, list), vec!["a", "b", "c", "x"]);
}

#[test]
fn a_reference_from_another_parent_is_refused() {
    let (mut document, list, _first, _second, _third) = list_of_three();
    let other = element(&mut document, "div");
    let stray = element(&mut document, "x");

    assert_eq!(
        document.insert_before(list, stray, Some(other)),
        Err(DomError::NotAChild)
    );
}

#[test]
fn clearing_children_empties_the_parent() {
    let (mut document, list, _first, _second, _third) = list_of_three();

    document.clear_children(list).unwrap();

    assert!(names(&document, list).is_empty());
    assert_eq!(document.node(list).unwrap().first_child(), None);
}

#[test]
fn setting_text_on_an_element_replaces_its_children() {
    let (mut document, list, _first, _second, _third) = list_of_three();

    document.set_text(list, "hello").unwrap();

    assert_eq!(text_content(&document, list), "hello");
    assert_eq!(document.children(list).count(), 1);
}

#[test]
fn setting_empty_text_clears_the_element() {
    let (mut document, list, _first, _second, _third) = list_of_three();

    document.set_text(list, "").unwrap();

    assert_eq!(document.children(list).count(), 0);
}

#[test]
fn setting_text_on_a_text_node_changes_it_in_place() {
    let mut document = Document::new();
    let text = document.create_text("old").unwrap();

    document.set_text(text, "new").unwrap();

    assert_eq!(document.node(text).unwrap().kind().as_text(), Some("new"));
}
