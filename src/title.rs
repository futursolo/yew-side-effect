//! A side effect that controls `document.title`.
//!
//! To use this side effect, you need to register [`TitleProvider`] like a React Context Provider
//! in your Application.
//!
//! Title can be set with [`Title`] Component.
//!
//! Only value provided to the last created [`Title`] will be set.
use std::rc::Rc;

use crate::{SideEffect, SideEffectProvider, SideEffects};
use gloo_utils::document;

use yew::prelude::*;

#[doc(hidden)]
#[derive(Debug, Clone, PartialEq)]
pub struct TitleSideEffect {
    value: String,
}

/// The Properties for Title Provider
#[derive(Properties, Clone)]
pub struct TitleProviderProps {
    /// The default title.
    pub default_title: String,

    /// A Function to format title.
    pub format_title: Rc<dyn Fn(&str) -> String>,

    pub children: Children,
}

#[allow(clippy::vtable_address_comparisons)]
impl PartialEq for TitleProviderProps {
    fn eq(&self, rhs: &Self) -> bool {
        self.default_title == rhs.default_title
            && self.children == rhs.children
            && Rc::ptr_eq(&self.format_title, &rhs.format_title)
    }
}

/// The Title Provider
///
/// You should register this title provider like a react context provider.
///
/// It accepts two props, a string `default_title` and a function `format_title`.
///
/// ```
/// use std::rc::Rc;
/// use yew::prelude::*;
/// use yew_side_effect::title::TitleProvider;
///
/// pub struct App;
///
/// impl Component for App {
///     type Message = ();
///     type Properties = ();
///
///     fn create(_ctx: &Context<Self>) -> Self {
///         Self
///     }
///
///     fn view(&self, _ctx: &Context<Self>) -> Html {
///         let children = Html::default();
///
///         let format_fn = Rc::new(|m: &str| format!("{} - My Site", m)) as Rc<dyn Fn(&str) -> String>;
///
///         html!{
///             <TitleProvider default_title="My Site" format_title={format_fn}>
///                 {children}
///             </TitleProvider>
///         }
///     }
/// }
/// ```
#[function_component(TitleProvider)]
pub fn title_provider(props: &TitleProviderProps) -> Html {
    let children = props.children.clone();
    let format_title = props.format_title.clone();
    let default_title = props.default_title.clone();

    let sync_title = Rc::new(move |titles: SideEffects<TitleSideEffect>| {
        // Set the last title to the document.
        let title = if let Some(m) = titles.last().map(|m| m.value.as_ref()) {
            format_title(m)
        } else {
            default_title.clone()
        };

        document().set_title(&title);
    }) as Rc<dyn Fn(SideEffects<TitleSideEffect>)>;

    html! {<SideEffectProvider<TitleSideEffect> on_change={sync_title}>{children}</SideEffectProvider<TitleSideEffect>>}
}

#[doc(hidden)]
#[derive(Properties, Clone, PartialEq)]
pub struct TitleProps {
    pub value: String,
}

/// Set a title
///
///
/// ```
/// use yew::prelude::*;
/// use yew_side_effect::title::Title;
///
/// let rendered = html! {<Title value="Homepage" />};
/// ```
#[function_component(Title)]
pub fn title(props: &TitleProps) -> Html {
    let effect = Rc::new(TitleSideEffect {
        value: props.value.clone(),
    });

    html! {<SideEffect<TitleSideEffect> value={effect} />}
}
