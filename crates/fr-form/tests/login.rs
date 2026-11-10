use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;
use fr_form::login::fill_login::fill_login;
use fr_form::login::find_login_fields::find_login_fields;
use fr_form::login::read_login::read_login;
use fr_form::state::read_value::read_value;
use fr_html::parse::parse_html::parse_html;

const LOGIN_PAGE: &str = "<form id=f action=/login method=post>\
    <input type=hidden name=csrf value=abc>\
    <input type=text id=user name=user>\
    <input type=password id=pass name=pass>\
    <button>Go</button></form>\
    <form id=search><input type=text id=q name=q></form>";

fn find(document: &Document, wanted: &str) -> NodeId {
    document
        .walk(document.root())
        .find_map(|edge| match edge {
            Edge::Enter(id) => {
                let element = document.node(id)?.kind().as_element()?;
                (element.attribute("id") == Some(wanted)).then_some(id)
            }
            Edge::Leave(_) => None,
        })
        .expect("element with the id")
}

#[test]
fn a_login_form_is_found_with_its_username_and_password() {
    let document = parse_html(LOGIN_PAGE);

    let found = find_login_fields(&document);

    assert_eq!(found.len(), 1);
    assert_eq!(found[0].password, find(&document, "pass"));
    assert_eq!(found[0].username, Some(find(&document, "user")));
    assert_eq!(found[0].form, Some(find(&document, "f")));
}

#[test]
fn a_page_without_a_password_field_has_no_login() {
    let document = parse_html("<form><input type=text name=q></form>");

    assert!(find_login_fields(&document).is_empty());
}

#[test]
fn the_username_must_be_in_the_same_form() {
    let document =
        parse_html("<form><input type=text id=a></form><form><input type=password id=p></form>");

    let found = find_login_fields(&document);

    assert_eq!(found.len(), 1);
    assert_eq!(found[0].username, None);
}

#[test]
fn filling_writes_both_fields() {
    let mut document = parse_html(LOGIN_PAGE);
    let fields = find_login_fields(&document)[0];

    assert!(fill_login(&mut document, &fields, "ada", "lovelace"));

    assert_eq!(read_value(&document, find(&document, "user")), "ada");
    assert_eq!(read_value(&document, find(&document, "pass")), "lovelace");
}

#[test]
fn filling_never_overwrites_what_is_already_typed() {
    let mut document =
        parse_html("<form><input type=text id=u value=typed><input type=password id=p></form>");
    let fields = find_login_fields(&document)[0];

    assert!(!fill_login(&mut document, &fields, "ada", "lovelace"));

    assert_eq!(read_value(&document, find(&document, "u")), "typed");
    assert_eq!(read_value(&document, find(&document, "p")), "");
}

#[test]
fn reading_a_submitted_login_gives_the_typed_values() {
    let mut document = parse_html(LOGIN_PAGE);
    let fields = find_login_fields(&document)[0];
    fill_login(&mut document, &fields, "ada", "lovelace");

    let form = find(&document, "f");

    assert_eq!(
        read_login(&document, form),
        Some(("ada".to_owned(), "lovelace".to_owned()))
    );
}

#[test]
fn a_form_with_an_empty_password_gives_no_login() {
    let document = parse_html(LOGIN_PAGE);

    assert_eq!(read_login(&document, find(&document, "f")), None);
    assert_eq!(read_login(&document, find(&document, "search")), None);
}
