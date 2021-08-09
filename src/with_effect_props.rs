use yew::prelude::*;

use crate::effects::Effects;

#[derive(Properties)]
pub struct WithEffectProps<T: Default + PartialEq + 'static> {
    pub effects: Effects<T>,
}

impl<T: Default + PartialEq + 'static> Clone for WithEffectProps<T> {
    fn clone(&self) -> Self {
        Self {
            effects: self.effects.clone(),
        }
    }
}

pub trait WithEffectPropsMut {
    type Effect: Default + PartialEq + 'static;

    fn effects_mut(&mut self) -> &mut Effects<Self::Effect>;
}

impl<T: Default + PartialEq + 'static> WithEffectPropsMut for WithEffectProps<T> {
    type Effect = T;

    fn effects_mut(&mut self) -> &mut Effects<T> {
        &mut self.effects
    }
}
