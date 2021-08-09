use std::rc::Rc;

use crate::effect::Effect;
use crate::effects::Effects;
use crate::utils::document;
use crate::with_effect::WithEffect;
use crate::with_effect_props::WithEffectPropsMut;

use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TitleEffect {
    value: Option<String>,
}

impl From<String> for TitleEffect {
    fn from(s: String) -> Self {
        Self { value: Some(s) }
    }
}

#[derive(Properties, Clone)]
pub struct TitleProviderProps {
    #[prop_or_default]
    pub effects: Effects<TitleEffect>,

    pub default_title: String,
    pub format_title: Rc<dyn Fn(&str) -> String>,

    pub children: Children,
}

impl WithEffectPropsMut for TitleProviderProps {
    type Effect = TitleEffect;

    fn effects_mut(&mut self) -> &mut Effects<TitleEffect> {
        &mut self.effects
    }
}

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
    fn sync_title(&self) {
        let title = if let Some(m) = self
            .props
            .effects
            .iter()
            .filter_map(|m| m.value.as_ref())
            .rev()
            .next()
        {
            (&*self.props.format_title)(m)
        } else {
            self.props.default_title.clone()
        };

        document().set_title(&title);
    }
}

pub type TitleProvider = WithEffect<BaseTitleProvider>;

#[derive(Properties, Clone, PartialEq)]
pub struct TitleProps {
    #[prop_or_default]
    pub value: Option<String>,
}

pub(crate) struct Title {
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
        let effect = Rc::new(TitleEffect {
            value: self.props.value.clone(),
        });

        html! {<Effect<TitleEffect> value=effect />}
    }
}
