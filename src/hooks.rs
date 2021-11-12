use yew::prelude::*;

use crate::store::Store;
use crate::SideEffects;

pub(crate) type StoreCtx<T> = UseReducerHandle<Store<T>>;

pub(crate) fn use_store<T>() -> Option<StoreCtx<T>>
where
    T: PartialEq + 'static,
{
    use_context::<UseReducerHandle<Store<T>>>()
}

/// A hook to read side effect.
pub fn use_side_effects<T>() -> Option<SideEffects<T>>
where
    T: PartialEq + 'static,
{
    use_store::<T>().map(|m| m.get())
}
