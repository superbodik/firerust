use std::time::Duration;

use fr_mem::meter::memory_meter::MemoryMeter;

#[test]
fn the_running_process_uses_some_memory() {
    let mut meter = MemoryMeter::new();

    let memory = meter.sample().expect("memory of this process");

    assert!(memory.resident_bytes > 0);
}

#[test]
fn a_fresh_sample_is_reused_within_its_age() {
    let mut meter = MemoryMeter::new();
    let first = meter.sample().expect("first sample");

    let second = meter
        .sample_if_stale(Duration::from_secs(60))
        .expect("second sample");

    assert_eq!(first, second);
}

#[test]
fn a_stale_request_takes_a_new_sample() {
    let mut meter = MemoryMeter::new();

    assert!(meter.sample_if_stale(Duration::ZERO).is_some());
}
