use std::rc::Rc;

use yew::prelude::*;

use crate::hooks::use_store;
use crate::store::Message;
use crate::utils::Id;

#[derive(Properties, PartialEq)]
pub struct SideEffectProps<T: PartialEq + 'static> {
    pub value: Rc<T>,
}

impl<T> Clone for SideEffectProps<T>
where
    T: PartialEq + 'static,
{
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}

/// Registers a single side effect.
///
/// Each `SideEffect<T>` accepts a `Rc<T>` as value, it will be stored in order of creation.
///
/// ```
/// use yew::prelude::*;
/// use std::rc::Rc;
/// use yew_side_effect::SideEffect;
///
/// #[derive(Debug, Clone, PartialEq)]
/// pub struct SideEffectA {
///     value: String,
/// }
///
/// let val = Rc::new(SideEffectA {
///     value: "My Side Effect!".into(),
/// });
///
/// let rendered = html! {<SideEffect<SideEffectA> value={val} />};
/// ```
#[function_component(SideEffect)]
pub fn side_effect<T>(props: &SideEffectProps<T>) -> Html
where
    T: PartialEq + 'static,
{
    let id = use_state(Id::new);
    let store = use_store().expect("No context set.");
    let store_id = store.id();


    let store_clone = store.clone();
    use_effect_with_deps(
        move |(_, value, id)| {
            if (*store_clone).has(id) {
                store_clone.dispatch(Message::Update((id.clone(), value.clone())));
            } else {
                store_clone.dispatch(Message::Add((id.clone(), value.clone())));
            }
            || {}
        },
        (store_id.clone(), props.value.clone(), (*id).clone()),
    );

    use_effect_with_deps(
        move |(_, id)| {
            let id = id.clone();

            move || store.dispatch(Message::Remove(id.clone()))
        },
        (store_id, (*id).clone()),
    );

    Html::default()
}
