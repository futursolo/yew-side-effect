use std::rc::Rc;

use yew::prelude::*;

use crate::SideEffects;

use crate::hooks::StoreCtx;
use crate::store::{Message, Store};

#[derive(Properties)]
pub struct ProviderProps<T: PartialEq + 'static> {
    pub children: Children,
    pub on_change: Rc<dyn Fn(SideEffects<T>)>,
}

#[allow(clippy::vtable_address_comparisons)]
impl<T> PartialEq for ProviderProps<T>
where
    T: PartialEq + 'static,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.children == rhs.children && Rc::ptr_eq(&self.on_change, &rhs.on_change)
    }
}

impl<T> Clone for ProviderProps<T>
where
    T: PartialEq + 'static,
{
    fn clone(&self) -> Self {
        Self {
            children: self.children.clone(),
            on_change: self.on_change.clone(),
        }
    }
}

/// Provider of a Side Effect.
///
/// You should register this title provider like a react context provider.
///
/// It accepts a function `on_change` which is called when side effect changes.
#[function_component(SideEffectProvider)]
pub fn side_effect_provider<T>(props: &ProviderProps<T>) -> Html
where
    T: PartialEq + 'static,
{
    let children = props.children.clone();

    let store = use_reducer_with_init(
        |prev: Rc<Store<T>>, action: Message<T>| {
            let mut store: Store<T> = (*prev).clone();
            store.reduce(action);

            store
        },
        (),
        |_| Store::new(),
    );

    use_effect_with_deps(
        |deps| {
            let (store, props) = deps;
            let on_change = props.on_change.clone();

            on_change(store.get());
            || {}
        },
        ((*store).clone(), props.to_owned()),
    );

    html! { <ContextProvider<StoreCtx<T>> context={store}>{children}</ContextProvider<StoreCtx<T>>> }
}
