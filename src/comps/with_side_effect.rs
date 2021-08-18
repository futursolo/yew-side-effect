use yew::agent::Bridge;
use yew::prelude::*;
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};

use crate::store::Store;
use crate::SideEffects;
use crate::WithEffectPropsMut;

type SideEffectType<T> = <<T as Component>::Properties as WithEffectPropsMut>::SideEffect;

pub enum WithSideEffectMsg<T: 'static> {
    Updated(ReadOnly<Store<T>>),
}

/// A Higher-Order Component to read side effects.
///
/// ```
/// use yew::prelude::*;
/// use yewtil::NeqAssign;
/// use yew_side_effect::{WithSideEffect, WithEffectProps};
///
/// #[derive(Debug, Clone, PartialEq)]
/// pub struct SideEffectA {
///     value: String,
/// }
///
/// pub struct BaseSideEffectAProvider {
///    props: WithEffectProps<SideEffectA>,
/// }
///
/// impl Component for BaseSideEffectAProvider {
///     type Message = ();
///     type Properties = WithEffectProps<SideEffectA>;
///
///     fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
///         Self { props }
///     }
///
///     fn update(&mut self, msg: Self::Message) -> ShouldRender {
///         false
///     }
///
///     fn change(&mut self, props: Self::Properties) -> ShouldRender {
///         self.props.neq_assign(props)
///     }
///
///     fn view(&self) -> Html {
///         html!{<div>{"Side Effects: "}{format!("{:?}", self.props.side_effects)}</div>}
///     }
/// }
///
/// type SideEffectAProvider = WithSideEffect<BaseSideEffectAProvider>;
/// ```
pub struct WithSideEffect<C>
where
    C: Component,
    C::Properties: WithEffectPropsMut,
{
    _store: Box<dyn Bridge<StoreWrapper<Store<SideEffectType<C>>>>>,

    props: C::Properties,

    effects: Option<SideEffects<SideEffectType<C>>>,
}

impl<C> Component for WithSideEffect<C>
where
    C: Component,
    C::Properties: WithEffectPropsMut,
{
    type Message = WithSideEffectMsg<SideEffectType<C>>;
    type Properties = C::Properties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Self::Message::Updated);

        let store = Store::bridge(callback);

        Self {
            props,
            _store: store,
            effects: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let Self::Message::Updated(m) = msg;

        self.effects = Some(m.borrow().get());

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
                    let effect_mut = props.side_effects_mut();
                    *effect_mut = m;
                }

                html! {<C with props />}
            }
            None => Html::default(),
        }
    }
}
