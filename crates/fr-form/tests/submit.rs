use fr_dom::node::node_id::NodeId;
use fr_dom::tree::document::Document;
use fr_dom::tree::edge::Edge;
use fr_form::submit::build_request::build_request;
use fr_form::submit::collect_fields::collect_fields;
use fr_form::submit::encode_form::encode_form;
use fr_form::submit::form_field::FormField;
use fr_form::submit::form_of::form_of;
use fr_form::submit::percent_encode::percent_encode;
use fr_html::parse::parse_html::parse_html;
use fr_net::load::request_method::RequestMethod;
use url::Url;

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

fn base() -> Url {
    Url::parse("https://example.test/dir/page.html").unwrap()
}

fn pairs(fields: &[FormField]) -> Vec<(String, String)> {
    fields
        .iter()
        .map(|field| (field.name.clone(), field.value.clone()))
        .collect()
}

#[test]
fn letters_digits_and_a_few_symbols_stay_as_they_are() {
    assert_eq!(percent_encode("Az09*-._"), "Az09*-._");
}

#[test]
fn spaces_become_plus_and_the_rest_is_percent_encoded() {
    assert_eq!(percent_encode("a b&c=d/é"), "a+b%26c%3Dd%2F%C3%A9");
}

#[test]
fn fields_are_joined_with_ampersands() {
    let fields = vec![FormField::new("a", "1"), FormField::new("b c", "x y")];

    assert_eq!(encode_form(&fields), "a=1&b+c=x+y");
}

#[test]
fn no_fields_give_an_empty_string() {
    assert_eq!(encode_form(&[]), "");
}

#[test]
fn a_control_knows_its_form() {
    let document = parse_html("<form id=f><div><input id=i></div></form><input id=lonely>");

    assert_eq!(
        form_of(&document, find(&document, "i")),
        Some(find(&document, "f"))
    );
    assert_eq!(form_of(&document, find(&document, "lonely")), None);
}

#[test]
fn text_fields_are_collected_in_order() {
    let document = parse_html(
        "<form id=f><input name=user value=ada><input type=password name=pass value=secret><input type=hidden name=token value=abc></form>",
    );

    let fields = collect_fields(&document, find(&document, "f"), None);

    assert_eq!(
        pairs(&fields),
        vec![
            ("user".to_owned(), "ada".to_owned()),
            ("pass".to_owned(), "secret".to_owned()),
            ("token".to_owned(), "abc".to_owned()),
        ]
    );
}

#[test]
fn controls_without_a_name_or_disabled_ones_are_skipped() {
    let document = parse_html(
        "<form id=f><input value=1><input name=a value=2 disabled><input name=b value=3></form>",
    );

    let fields = collect_fields(&document, find(&document, "f"), None);

    assert_eq!(pairs(&fields), vec![("b".to_owned(), "3".to_owned())]);
}

#[test]
fn only_checked_boxes_and_radios_are_sent() {
    let document = parse_html(
        "<form id=f><input type=checkbox name=a checked><input type=checkbox name=b value=yes><input type=checkbox name=c value=yes checked><input type=radio name=r value=1><input type=radio name=r value=2 checked></form>",
    );

    let fields = collect_fields(&document, find(&document, "f"), None);

    assert_eq!(
        pairs(&fields),
        vec![
            ("a".to_owned(), "on".to_owned()),
            ("c".to_owned(), "yes".to_owned()),
            ("r".to_owned(), "2".to_owned()),
        ]
    );
}

#[test]
fn a_textarea_sends_its_text() {
    let document = parse_html("<form id=f><textarea name=note>hello\nworld</textarea></form>");

    let fields = collect_fields(&document, find(&document, "f"), None);

    assert_eq!(
        pairs(&fields),
        vec![("note".to_owned(), "hello\nworld".to_owned())]
    );
}

#[test]
fn a_select_sends_the_selected_option_or_the_first_one() {
    let document = parse_html(
        "<form id=f><select name=a><option value=x>X</option><option value=y selected>Y</option></select><select name=b><option>First</option><option>Second</option></select></form>",
    );

    let fields = collect_fields(&document, find(&document, "f"), None);

    assert_eq!(
        pairs(&fields),
        vec![
            ("a".to_owned(), "y".to_owned()),
            ("b".to_owned(), "First".to_owned()),
        ]
    );
}

#[test]
fn only_the_pressed_submit_button_is_sent() {
    let document = parse_html(
        "<form id=f><input type=submit id=one name=action value=save><input type=submit id=two name=action value=delete></form>",
    );
    let form = find(&document, "f");

    let with_two = collect_fields(&document, form, Some(find(&document, "two")));
    let without = collect_fields(&document, form, None);

    assert_eq!(
        pairs(&with_two),
        vec![("action".to_owned(), "delete".to_owned())]
    );
    assert!(without.is_empty());
}

#[test]
fn a_get_form_puts_the_fields_in_the_query() {
    let document = parse_html(
        "<form id=f action='/search' method=get><input name=q value='rust lang'><input name=page value=2></form>",
    );

    let request = build_request(&document, find(&document, "f"), None, &base()).unwrap();

    assert_eq!(request.method, RequestMethod::Get);
    assert_eq!(
        request.url.as_str(),
        "https://example.test/search?q=rust+lang&page=2"
    );
    assert!(request.body.is_empty());
}

#[test]
fn the_default_method_is_get() {
    let document = parse_html("<form id=f action='/s'><input name=q value=x></form>");

    let request = build_request(&document, find(&document, "f"), None, &base()).unwrap();

    assert_eq!(request.method, RequestMethod::Get);
}

#[test]
fn a_post_form_sends_an_encoded_body() {
    let document = parse_html(
        "<form id=f action='login' method='POST'><input name=user value=ada><input name=pass value='p&w'></form>",
    );

    let request = build_request(&document, find(&document, "f"), None, &base()).unwrap();

    assert_eq!(request.method, RequestMethod::Post);
    assert_eq!(request.url.as_str(), "https://example.test/dir/login");
    assert_eq!(request.body, b"user=ada&pass=p%26w");
    assert_eq!(
        request.content_type.as_deref(),
        Some("application/x-www-form-urlencoded")
    );
}

#[test]
fn a_missing_action_submits_to_the_page_itself() {
    let document = parse_html("<form id=f method=post><input name=a value=1></form>");

    let request = build_request(&document, find(&document, "f"), None, &base()).unwrap();

    assert_eq!(request.url.as_str(), "https://example.test/dir/page.html");
}

#[test]
fn an_old_query_is_replaced_for_get_forms() {
    let document = parse_html("<form id=f action='/s?old=1'><input name=q value=x></form>");

    let request = build_request(&document, find(&document, "f"), None, &base()).unwrap();

    assert_eq!(request.url.as_str(), "https://example.test/s?q=x");
}

#[test]
fn an_empty_get_form_has_no_query() {
    let document = parse_html("<form id=f action='/s?old=1'></form>");

    let request = build_request(&document, find(&document, "f"), None, &base()).unwrap();

    assert_eq!(request.url.as_str(), "https://example.test/s");
}
