use std::time::{Duration, Instant};

pub struct Notice {
    pub message: String,
    shown: Instant,
    lifetime: Duration,
}

impl Notice {
    pub fn new(message: &str, lifetime: Duration) -> Self {
        Self {
            message: message.to_owned(),
            shown: Instant::now(),
            lifetime,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.shown.elapsed() >= self.lifetime
    }

    pub fn remaining(&self) -> Duration {
        self.lifetime.saturating_sub(self.shown.elapsed())
    }
}
