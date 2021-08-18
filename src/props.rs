use yew::prelude::*;

use crate::SideEffects;

/// A Convenient Properties for a `Provider` to access the side effects.
#[derive(Properties, Debug)]
pub struct WithEffectProps<T> {
    pub side_effects: SideEffects<T>,
}

impl<T> Clone for WithEffectProps<T> {
    fn clone(&self) -> Self {
        Self {
            side_effects: self.side_effects.clone(),
        }
    }
}

impl<T: PartialEq> PartialEq for WithEffectProps<T> {
    fn eq(&self, other: &Self) -> bool {
        self.side_effects == other.side_effects
    }
}

/// A trait for a `Provider` to access the side effects.
///
/// Any struct that implements this trait can be used to as `Properties` to access side effects
/// with [`WithSideEffect`](crate::WithSideEffect).
pub trait WithEffectPropsMut {
    type SideEffect;

    /// Returns a mutable reference of [`SideEffects`].
    fn side_effects_mut(&mut self) -> &mut SideEffects<Self::SideEffect>;
}

impl<T> WithEffectPropsMut for WithEffectProps<T> {
    type SideEffect = T;

    fn side_effects_mut(&mut self) -> &mut SideEffects<T> {
        &mut self.side_effects
    }
}
