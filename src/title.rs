//! A side effect that controls `document.title`.
//!
//! To use this side effect, you need to register [`TitleProvider`] like a React Context Provider
//! in your Application.
//!
//! Title can be set with [`Title`] Component.
//!
//! Only value provided to the last created [`Title`] will be set.
use std::rc::Rc;

use crate::SideEffect;
use crate::SideEffects;
use crate::WithEffectPropsMut;
use crate::WithSideEffect;
use yew::utils::document;

use yew::prelude::*;
use yewtil::NeqAssign;

#[doc(hidden)]
#[derive(Debug, Clone, PartialEq)]
pub struct TitleSideEffect {
    value: String,
}

/// The Properties for Title Provider
#[derive(Properties, Clone)]
pub struct TitleProviderProps {
    #[doc(hidden)]
    #[prop_or_default]
    pub side_effects: SideEffects<TitleSideEffect>,

    /// The default title.
    pub default_title: String,

    /// A Function to format title.
    pub format_title: Rc<dyn Fn(&str) -> String>,

    pub children: Children,
}

impl WithEffectPropsMut for TitleProviderProps {
    type SideEffect = TitleSideEffect;

    fn side_effects_mut(&mut self) -> &mut SideEffects<TitleSideEffect> {
        &mut self.side_effects
    }
}

/// The base [`TitleProvider`] without [`WithSideEffect`]
pub struct BaseTitleProvider {
    props: TitleProviderProps,
}

impl Component for BaseTitleProvider {
    type Message = ();
    type Properties = TitleProviderProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.sync_title();
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;

        self.sync_title();

        true
    }

    fn view(&self) -> Html {
        html! {<>{self.props.children.clone()}</>}
    }
}

impl BaseTitleProvider {
    // We set the last title to the document.
    fn sync_title(&self) {
        let title = if let Some(m) = self.props.side_effects.last().map(|m| m.value.as_ref()) {
            (&*self.props.format_title)(m)
        } else {
            self.props.default_title.clone()
        };

        document().set_title(&title);
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
///     fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
///         Self
///     }
///
///     fn update(&mut self, _msg: Self::Message) -> ShouldRender {
///         false
///     }
///
///     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
///         false
///     }
///
///     fn view(&self) -> Html {
///         let children = Html::default();
///
///         let format_fn = Rc::new(|m: &str| format!("{} - My Site", m)) as Rc<dyn Fn(&str) -> String>;
///
///         html!{
///             <TitleProvider default_title="My Site" format_title=format_fn>
///                 {children}
///             </TitleProvider>
///         }
///     }
/// }
/// ```
pub type TitleProvider = WithSideEffect<BaseTitleProvider>;

#[doc(hidden)]
#[derive(Properties, Clone, PartialEq)]
pub struct TitleProps {
    #[prop_or_default]
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
pub struct Title {
    props: TitleProps,
}

impl Component for Title {
    type Message = ();
    type Properties = TitleProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let effect = Rc::new(TitleSideEffect {
            value: self.props.value.clone(),
        });

        html! {<SideEffect<TitleSideEffect> value=effect />}
    }
}
