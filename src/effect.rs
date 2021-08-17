use std::rc::Rc;

use yew::agent::Bridge;
use yew::prelude::*;
use yewtil::store::{Bridgeable, StoreWrapper};

use crate::store;
use crate::utils::Id;

#[derive(Properties)]
pub struct EffectProps<T: 'static> {
    pub value: Rc<T>,
}

impl<T: 'static> Clone for EffectProps<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}

pub struct Effect<T: 'static> {
    props: EffectProps<T>,
    id: Id,
    store: Box<dyn Bridge<StoreWrapper<store::EffectStore<T>>>>,
}

impl<T: 'static> Component for Effect<T> {
    type Message = ();
    type Properties = EffectProps<T>;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let id = Id::new();
        let mut store = store::EffectStore::bridge(Callback::from(|_| ()));

        store.send(store::Message::Add((id.clone(), props.value.clone())));

        Self {
            props,
            // link,
            id,
            store, // callback,
        }
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
