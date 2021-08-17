use yew::agent::Bridge;
use yew::prelude::*;
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};

use crate::effects::Effects;
use crate::store::EffectStore;
use crate::with_effect_props::WithEffectPropsMut;

type EffectType<T> = <<T as Component>::Properties as WithEffectPropsMut>::Effect;

pub enum WithEffectMsg<T: 'static> {
    EffectUpdated(ReadOnly<EffectStore<T>>),
}

pub struct WithEffect<C>
where
    C: Component,
    C::Properties: WithEffectPropsMut,
{
    _store: Box<dyn Bridge<StoreWrapper<EffectStore<EffectType<C>>>>>,

    props: C::Properties,

    effects: Option<Effects<EffectType<C>>>,
}

impl<C> Component for WithEffect<C>
where
    C: Component,
    C::Properties: WithEffectPropsMut,
{
    type Message = WithEffectMsg<EffectType<C>>;
    type Properties = C::Properties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(WithEffectMsg::EffectUpdated);

        let store = EffectStore::bridge(callback);

        Self {
            props,
            _store: store,
            effects: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let WithEffectMsg::EffectUpdated(m) = msg;

        self.effects = Some(m.borrow().get_effects());

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match self.effects.clone() {
            Some(m) => {
                let mut props = self.props.clone();
                {
                    let effect_mut = props.effects_mut();
                    *effect_mut = m;
                }

                html! {<C with props />}
            }
            None => Html::default(),
        }
    }
}
