#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(non_snake_case)]
//! Reconcile Side Effects in Yew Applications
//!
//! This library is inspired by [react-side-effect](https://github.com/gaearon/react-side-effect)
//! and [react-helmet](https://github.com/nfl/react-helmet).
//!
//! # Usage
//!
//! 1. Define a SideEffectType
//!
//! Each side effect needs to have a different type(struct).
//!
//! 2. Set Side Effects with [`SideEffect<SideEffectType>`]
//!
//! All side effects will be collected at component's creation order.
//!
//! 3. Read Side Effects using [`WithSideEffect<T>`]
//!
//! Please refer to source code of [`title`] for a complete example.

mod collections;
mod comps;
mod props;
mod store;
pub mod title;
mod utils;

pub use comps::{SideEffect, WithSideEffect};

pub use collections::SideEffects;

pub use props::{WithEffectProps, WithEffectPropsMut};
