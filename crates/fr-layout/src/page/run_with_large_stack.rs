const STACK_BYTES: usize = 256 * 1024 * 1024;

pub fn run_with_large_stack<T: Send>(mut work: impl FnMut() -> T + Send) -> T {
    let outcome = std::thread::scope(|scope| {
        let spawned = std::thread::Builder::new()
            .stack_size(STACK_BYTES)
            .spawn_scoped(scope, &mut work);
        match spawned {
            Ok(handle) => Some(handle.join()),
            Err(_) => None,
        }
    });
    match outcome {
        Some(Ok(value)) => value,
        Some(Err(payload)) => std::panic::resume_unwind(payload),
        None => work(),
    }
}
