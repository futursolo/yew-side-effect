use yew::prelude::*;

use crate::store::{Message, Store};

pub(crate) type StoreCtx<T> = UseReducerHandle<Store<T>, Message<T>>;

pub(crate) fn use_store<T>() -> Option<StoreCtx<T>>
where
    T: PartialEq + 'static,
{
    use_context::<UseReducerHandle<Store<T>, Message<T>>>()
}
