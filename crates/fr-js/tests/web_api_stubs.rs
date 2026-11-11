mod support;

use support::Page;

#[test]
fn new_image_makes_an_img_element_with_the_given_size() {
    let mut page = Page::new("<p></p>");

    page.run("var a = new Image(); var b = new Image(10, 20); console.log(a.tagName, b.getAttribute('width'), b.getAttribute('height'))");

    assert_eq!(page.logs(), vec!["[log] IMG 10 20"]);
}

#[test]
fn an_image_can_be_given_a_source_and_added_to_the_page() {
    let mut page = Page::new("<div id=box></div>");

    page.run("var i = new Image(); i.setAttribute('src', 'a.png'); document.getElementById('box').appendChild(i)");

    assert_eq!(page.document.borrow().children(page.id("box")).count(), 1);
}

#[test]
fn a_message_sent_on_one_port_arrives_on_the_other_later() {
    let mut page = Page::new("<p></p>");

    page.run(
        "var channel = new MessageChannel();\
         channel.port1.onmessage = function (event) { console.log('got', event.data); };\
         channel.port2.postMessage('hello'); console.log('sent');",
    );

    assert_eq!(page.logs(), vec!["[log] sent"]);
    page.runtime.advance_time(10.0).unwrap();
    assert_eq!(page.logs(), vec!["[log] got hello"]);
}

#[test]
fn listeners_added_with_add_event_listener_also_receive_messages() {
    let mut page = Page::new("<p></p>");

    page.run(
        "var channel = new MessageChannel();\
         channel.port2.addEventListener('message', function (event) { console.log('port2', event.data); });\
         channel.port1.postMessage(42);",
    );
    page.runtime.advance_time(10.0).unwrap();

    assert_eq!(page.logs(), vec!["[log] port2 42"]);
}

#[test]
fn a_closed_port_sends_nothing() {
    let mut page = Page::new("<p></p>");

    page.run(
        "var channel = new MessageChannel();\
         channel.port1.onmessage = function () { console.log('unexpected'); };\
         channel.port2.close(); channel.port2.postMessage('x');",
    );
    page.runtime.advance_time(10.0).unwrap();

    assert!(page.logs().is_empty());
}
