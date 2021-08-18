use std::rc::Rc;

use yew::agent::Bridge;
use yew::prelude::*;
use yewtil::store::{Bridgeable, StoreWrapper};

use crate::store;
use crate::utils::Id;

#[derive(Properties)]
pub struct SideEffectProps<T: 'static> {
    pub value: Rc<T>,
}

impl<T> Clone for SideEffectProps<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}

/// Registers a signle side effect.
///
/// Each `SideEffect<T>` accepts a `Rc<T>` as value, it will be stored in order of creation.
///
/// ```
/// use yew::prelude::*;
/// use std::rc::Rc;
/// use yew_side_effect::SideEffect;
///
/// #[derive(Debug, Clone)]
/// pub struct SideEffectA {
///     value: String,
/// }
///
/// let val = Rc::new(SideEffectA {
///     value: "My Side Effect!".into(),
/// });
///
/// let rendered = html! {<SideEffect<SideEffectA> value=val />};
/// ```
pub struct SideEffect<T: 'static> {
    props: SideEffectProps<T>,
    id: Id,
    store: Box<dyn Bridge<StoreWrapper<store::Store<T>>>>,
}

impl<T> Component for SideEffect<T> {
    type Message = ();
    type Properties = SideEffectProps<T>;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let id = Id::new();
        let mut store = store::Store::bridge(Callback::from(|_| ()));

        store.send(store::Message::Add((id.clone(), props.value.clone())));

        Self { props, id, store }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;

        self.store.send(store::Message::Update((
            self.id.clone(),
            self.props.value.clone(),
        )));

        false
    }

    fn destroy(&mut self) {
        self.store.send(store::Message::Remove(self.id.clone()));
    }

    fn view(&self) -> Html {
        Html::default()
    }
}
