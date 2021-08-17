use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(u64);

impl Id {
    pub fn new() -> Self {
        static CTR: Lazy<Arc<Mutex<u64>>> = Lazy::new(Arc::default);

        let counter = CTR.clone();
        let mut counter = counter.lock().unwrap();

        *counter += 1;
        Self(*counter)
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}
