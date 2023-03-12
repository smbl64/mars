use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Default)]
pub struct IdGenerator {
    value: AtomicU64,
}

impl IdGenerator {
    pub fn next(&mut self) -> u64 {
        self.value.fetch_add(1, Ordering::Relaxed)
    }
}
