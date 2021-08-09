use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
use yew::web_sys::{Document, Window};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(u64);

impl Id {
    pub fn new() -> Self {
        static CTR: Lazy<Arc<Mutex<u64>>> = Lazy::new(|| Arc::default());

        let counter = CTR.clone();
        let mut counter = counter.lock().unwrap();

        *counter += 1;
        Self(*counter)
    }
}

pub(crate) fn window() -> Window {
    yew::web_sys::window().expect("Window is not available.")
}

pub(crate) fn document() -> Document {
    window().document().expect("Document is not available.")
}
