use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;

#[test]
fn walk_visits_every_node_in_document_order() {
    let mut document = Document::new();
    let root = document.root();
    let div = document.create_element("div", Vec::new()).expect("div");
    let text = document.create_text("a").expect("text");
    let span = document.create_element("span", Vec::new()).expect("span");
    document.append_child(root, div).expect("div into root");
    document.append_child(div, text).expect("text into div");
    document.append_child(div, span).expect("span into div");

    let edges: Vec<_> = document.walk(root).collect();

    assert_eq!(
        edges,
        vec![
            Edge::Enter(root),
            Edge::Enter(div),
            Edge::Enter(text),
            Edge::Leave(text),
            Edge::Enter(span),
            Edge::Leave(span),
            Edge::Leave(div),
            Edge::Leave(root),
        ]
    );
}

#[test]
fn walk_of_a_subtree_stops_at_its_root() {
    let mut document = Document::new();
    let root = document.root();
    let first = document.create_element("p", Vec::new()).expect("first");
    let second = document.create_element("p", Vec::new()).expect("second");
    document.append_child(root, first).expect("first into root");
    document
        .append_child(root, second)
        .expect("second into root");

    let edges: Vec<_> = document.walk(first).collect();

    assert_eq!(edges, vec![Edge::Enter(first), Edge::Leave(first)]);
}

#[test]
fn walk_survives_very_deep_trees() {
    let mut document = Document::new();
    let mut parent = document.root();
    for _ in 0..50_000 {
        let child = document.create_element("div", Vec::new()).expect("child");
        document.append_child(parent, child).expect("append");
        parent = child;
    }

    let visited = document.walk(document.root()).count();

    assert_eq!(visited, 2 * document.node_count());
}
