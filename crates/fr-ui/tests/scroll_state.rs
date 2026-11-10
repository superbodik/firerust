use fr_ui::scroll::scroll_metrics::ScrollMetrics;
use fr_ui::scroll::scroll_state::ScrollState;
use fr_window::input::key_input::KeyInput;

fn tall() -> ScrollMetrics {
    ScrollMetrics::new(2000.0, 600.0)
}

#[test]
fn scrolling_down_moves_the_offset() {
    let mut state = ScrollState::new();

    assert!(state.scroll_by(100.0, tall()));
    assert_eq!(state.offset(), 100.0);
}

#[test]
fn the_offset_never_goes_above_the_top() {
    let mut state = ScrollState::new();

    assert!(!state.scroll_by(-50.0, tall()));
    assert_eq!(state.offset(), 0.0);
}

#[test]
fn the_offset_never_goes_below_the_bottom() {
    let mut state = ScrollState::new();

    state.scroll_by(99999.0, tall());

    assert_eq!(state.offset(), 1400.0);
}

#[test]
fn a_short_page_cannot_scroll() {
    let mut state = ScrollState::new();

    assert!(!state.scroll_by(100.0, ScrollMetrics::new(300.0, 600.0)));
    assert_eq!(state.offset(), 0.0);
}

#[test]
fn fractional_offsets_are_rounded_to_whole_pixels() {
    let mut state = ScrollState::new();

    state.scroll_by(10.4, tall());

    assert_eq!(state.offset(), 10.0);
}

#[test]
fn clamping_pulls_the_offset_back_after_the_page_shrinks() {
    let mut state = ScrollState::new();
    state.scroll_to(1400.0, tall());

    assert!(state.clamp(ScrollMetrics::new(1000.0, 600.0)));
    assert_eq!(state.offset(), 400.0);
}

#[test]
fn arrow_keys_scroll_by_a_fixed_step() {
    let mut state = ScrollState::new();

    state.apply_key(KeyInput::ArrowDown, tall());
    assert_eq!(state.offset(), 48.0);
    state.apply_key(KeyInput::ArrowUp, tall());
    assert_eq!(state.offset(), 0.0);
}

#[test]
fn page_keys_scroll_almost_a_viewport() {
    let mut state = ScrollState::new();

    state.apply_key(KeyInput::PageDown, tall());
    assert_eq!(state.offset(), 540.0);
    state.apply_key(KeyInput::PageUp, tall());
    assert_eq!(state.offset(), 0.0);
}

#[test]
fn space_acts_like_page_down() {
    let mut state = ScrollState::new();

    state.apply_key(KeyInput::Space, tall());

    assert_eq!(state.offset(), 540.0);
}

#[test]
fn home_and_end_jump_to_the_edges() {
    let mut state = ScrollState::new();

    state.apply_key(KeyInput::End, tall());
    assert_eq!(state.offset(), 1400.0);
    state.apply_key(KeyInput::Home, tall());
    assert_eq!(state.offset(), 0.0);
}
