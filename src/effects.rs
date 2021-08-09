use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct Effects<T> {
    inner: Vec<Rc<T>>,
}

impl<T: Default + PartialEq + 'static> Deref for Effects<T> {
    type Target = [Rc<T>];

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl<T: Default + PartialEq + 'static> Effects<T> {
    pub fn new(from: Vec<Rc<T>>) -> Self {
        Self { inner: from }
    }
}

impl<T: Default + PartialEq + 'static> Clone for Effects<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
