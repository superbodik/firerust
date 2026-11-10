mod support;

use support::Page;

#[test]
fn a_timeout_fires_when_its_time_has_come() {
    let mut page = Page::new("<p></p>");
    page.run("setTimeout(function () { console.log('later'); }, 100);");

    page.runtime.advance_time(99.0).unwrap();
    assert!(page.logs().is_empty());
    page.runtime.advance_time(100.0).unwrap();

    assert_eq!(page.logs(), vec!["[log] later"]);
}

#[test]
fn a_timeout_fires_only_once() {
    let mut page = Page::new("<p></p>");
    page.run("setTimeout(function () { console.log('once'); }, 10);");

    page.runtime.advance_time(50.0).unwrap();
    page.runtime.advance_time(500.0).unwrap();

    assert_eq!(page.logs(), vec!["[log] once"]);
}

#[test]
fn timeouts_fire_in_order_of_their_due_time() {
    let mut page = Page::new("<p></p>");
    page.run("setTimeout(function () { console.log('b'); }, 20); setTimeout(function () { console.log('a'); }, 10); setTimeout(function () { console.log('c'); }, 20);");

    page.runtime.advance_time(100.0).unwrap();

    assert_eq!(page.logs(), vec!["[log] a", "[log] b", "[log] c"]);
}

#[test]
fn extra_arguments_reach_the_callback() {
    let mut page = Page::new("<p></p>");
    page.run("setTimeout(function (a, b) { console.log(a + b); }, 1, 2, 3);");

    page.runtime.advance_time(5.0).unwrap();

    assert_eq!(page.logs(), vec!["[log] 5"]);
}

#[test]
fn an_interval_repeats() {
    let mut page = Page::new("<p></p>");
    page.run("var n = 0; setInterval(function () { n += 1; console.log('tick', n); }, 100);");

    page.runtime.advance_time(350.0).unwrap();

    assert_eq!(
        page.logs(),
        vec!["[log] tick 1", "[log] tick 2", "[log] tick 3"]
    );
}

#[test]
fn clearing_a_timeout_cancels_it() {
    let mut page = Page::new("<p></p>");
    page.run("var id = setTimeout(function () { console.log('never'); }, 10); clearTimeout(id);");

    page.runtime.advance_time(100.0).unwrap();

    assert!(page.logs().is_empty());
}

#[test]
fn an_interval_can_stop_itself() {
    let mut page = Page::new("<p></p>");
    page.run("var n = 0; var id = setInterval(function () { n += 1; console.log(n); if (n === 2) { clearInterval(id); } }, 10);");

    page.runtime.advance_time(1000.0).unwrap();

    assert_eq!(page.logs(), vec!["[log] 1", "[log] 2"]);
}

#[test]
fn a_timer_can_schedule_another_one() {
    let mut page = Page::new("<p></p>");
    page.run("setTimeout(function () { console.log('first'); setTimeout(function () { console.log('second'); }, 10); }, 10);");

    page.runtime.advance_time(15.0).unwrap();
    assert_eq!(page.logs(), vec!["[log] first"]);
    page.runtime.advance_time(25.0).unwrap();

    assert_eq!(page.logs(), vec!["[log] second"]);
}

#[test]
fn the_next_due_time_is_reported() {
    let mut page = Page::new("<p></p>");

    assert_eq!(page.runtime.next_timer_due(), None);
    page.run("setTimeout(function () {}, 300); setTimeout(function () {}, 120);");

    assert_eq!(page.runtime.next_timer_due(), Some(120.0));
}

#[test]
fn timers_are_measured_from_the_current_time() {
    let mut page = Page::new("<p></p>");
    page.runtime.advance_time(1000.0).unwrap();

    page.run("setTimeout(function () { console.log('x'); }, 50);");

    assert_eq!(page.runtime.next_timer_due(), Some(1050.0));
}

#[test]
fn an_error_in_a_timer_is_logged_and_the_rest_continue() {
    let mut page = Page::new("<p></p>");
    page.run("setTimeout(function () { throw new Error('oops'); }, 1); setTimeout(function () { console.log('after'); }, 2);");

    page.runtime.advance_time(10.0).unwrap();

    let logs = page.logs();
    assert!(logs[0].starts_with("[error] Uncaught Error: oops"));
    assert_eq!(logs[1], "[log] after");
}

#[test]
fn request_animation_frame_runs_with_the_time() {
    let mut page = Page::new("<p></p>");
    page.run("requestAnimationFrame(function (t) { console.log('frame', t); });");

    page.runtime.advance_time(20.0).unwrap();

    assert_eq!(page.logs(), vec!["[log] frame 20"]);
}

#[test]
fn performance_now_follows_the_clock() {
    let mut page = Page::new("<p></p>");
    page.runtime.advance_time(250.0).unwrap();

    page.run("console.log(performance.now())");

    assert_eq!(page.logs(), vec!["[log] 250"]);
}

#[test]
fn a_runaway_interval_does_not_hang_the_runtime() {
    let mut page = Page::new("<p></p>");
    page.run("var n = 0; setInterval(function () { n += 1; }, 1);");

    page.runtime.advance_time(1_000_000.0).unwrap();

    page.run("console.log(n > 0)");
    assert_eq!(page.logs(), vec!["[log] true"]);
}
