use std::ops::Deref;
use std::rc::Rc;

/// A collection of all registered side effect values of a certain side effect.
///
/// This struct can be dereferenced to an `&[Rc<T>]`. You can use it like a normal slice.
#[derive(Debug)]
pub struct SideEffects<T> {
    inner: Vec<Rc<T>>,
}

impl<T> Deref for SideEffects<T> {
    type Target = [Rc<T>];

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl<T> SideEffects<T> {
    pub(crate) fn new(from: Vec<Rc<T>>) -> Self {
        Self { inner: from }
    }
}

impl<T> Clone for SideEffects<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Default for SideEffects<T> {
    fn default() -> Self {
        Self { inner: Vec::new() }
    }
}

impl<T: PartialEq> PartialEq for SideEffects<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}
