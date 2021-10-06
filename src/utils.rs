use std::sync::atomic::{AtomicU64, Ordering};

use once_cell::sync::Lazy;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(u64);

impl Id {
    pub fn new() -> Self {
        static CTR: Lazy<AtomicU64> = Lazy::new(AtomicU64::default);

        Self(CTR.fetch_add(1, Ordering::SeqCst))
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}
