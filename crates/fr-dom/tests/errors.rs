use fr_dom::error::dom_error::DomError;
use fr_dom::tree::document::Document;

#[test]
fn a_node_cannot_be_attached_twice() {
    let mut document = Document::new();
    let root = document.root();
    let child = document.create_element("p", Vec::new()).expect("child");
    document.append_child(root, child).expect("first attach");

    assert_eq!(
        document.append_child(root, child),
        Err(DomError::AlreadyAttached)
    );
}

#[test]
fn the_root_cannot_be_attached() {
    let mut document = Document::new();
    let root = document.root();
    let child = document.create_element("p", Vec::new()).expect("child");

    assert_eq!(
        document.append_child(child, root),
        Err(DomError::AlreadyAttached)
    );
}

#[test]
fn a_node_from_another_document_is_unknown() {
    let mut document = Document::new();
    let mut other = Document::new();
    other.create_text("a").expect("first");
    let foreign = other.create_text("b").expect("foreign");

    assert_eq!(
        document.append_child(document.root(), foreign),
        Err(DomError::UnknownNode)
    );
}

#[test]
fn a_cycle_is_refused() {
    let mut document = Document::new();
    let outer = document.create_element("div", Vec::new()).expect("outer");
    let inner = document.create_element("div", Vec::new()).expect("inner");
    document
        .append_child(outer, inner)
        .expect("inner into outer");

    assert_eq!(
        document.append_child(inner, outer),
        Err(DomError::WouldCreateCycle)
    );
}

#[test]
fn a_node_cannot_contain_itself() {
    let mut document = Document::new();
    let node = document.create_element("div", Vec::new()).expect("node");

    assert_eq!(
        document.append_child(node, node),
        Err(DomError::WouldCreateCycle)
    );
}
