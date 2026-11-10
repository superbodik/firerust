use fr_dom::node::attribute::Attribute;
use fr_dom::tree::document::Document;

#[test]
fn children_keep_insertion_order() {
    let mut document = Document::new();
    let root = document.root();
    let first = document.create_element("p", Vec::new()).expect("first");
    let second = document.create_element("div", Vec::new()).expect("second");
    let third = document.create_text("text").expect("third");

    document.append_child(root, first).expect("append first");
    document.append_child(root, second).expect("append second");
    document.append_child(root, third).expect("append third");

    let ids: Vec<_> = document.children(root).collect();
    assert_eq!(ids, vec![first, second, third]);
    assert_eq!(
        document.node(second).and_then(|node| node.prev_sibling()),
        Some(first)
    );
    assert_eq!(
        document.node(second).and_then(|node| node.next_sibling()),
        Some(third)
    );
    assert_eq!(
        document.node(third).and_then(|node| node.parent()),
        Some(root)
    );
}

#[test]
fn element_exposes_attributes_by_name() {
    let mut document = Document::new();
    let link = document
        .create_element(
            "a",
            vec![Attribute::new("href", "/next"), Attribute::new("id", "go")],
        )
        .expect("link");

    let data = document
        .node(link)
        .and_then(|node| node.kind().as_element())
        .expect("element");

    assert_eq!(data.name(), "a");
    assert_eq!(data.attribute("href"), Some("/next"));
    assert_eq!(data.attribute("id"), Some("go"));
    assert_eq!(data.attribute("class"), None);
}

#[test]
fn text_node_exposes_its_text() {
    let mut document = Document::new();
    let text = document.create_text("hello").expect("text");

    let kind = document.node(text).map(|node| node.kind()).expect("node");

    assert_eq!(kind.as_text(), Some("hello"));
    assert!(kind.as_element().is_none());
}

#[test]
fn node_count_includes_the_root() {
    let mut document = Document::new();
    assert_eq!(document.node_count(), 1);
    document.create_text("a").expect("text");
    assert_eq!(document.node_count(), 2);
}
