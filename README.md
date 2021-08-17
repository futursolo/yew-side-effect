# yew-side-effect

Reconcile Side Effects in Yew Applications

This library is inspired by [react-side-effect](https://github.com/gaearon/react-side-effect)
and [react-helmet](https://github.com/nfl/react-helmet).

## Usage

1. Define an `EffectType`

`yew-side-effect` uses `yewtil::store` internally to collect effects.
Each effect needs to have a different struct.

```rust
#[derive(Debug, Clone)]
pub struct EffectA {
    value: String,
}
```

2. Register effects using `<Effect<EffectType> />`

Each `<Effect<EffectType> />` accepts a value with type
`Rc<EffectType>`.

```rust
let effect = Rc::new(EffectA {
    value: "Some value".into(),
});


html! {<Effect<EffectA> value=effect />}
```

Effects are registered in rendering order.

3. Reconcile effects with `<WithEffect<Provider> />`

Define a `Provider` component that reconciles effects.

`Provider` component needs to have a prop with type `WithEffectProps` or
have a type that implements `WithEffectPropsMut`.

There can be multiple consumers for a signle effect.


```rust
pub struct BaseEffectApplier {
    props: WithEffectProps<EffectA>,
}

impl Component for BaseEffectApplier {
    type Message = ();
    type Properties = WithEffectProps<EffectA>;

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
        Html::default()
    }
}

pub type EffectApplier = WithEffect<BaseEffectApplier>;
```

This HOC pattern is inspired by Yewdux.

`WithEffectProps` has a property effects, which dereferences to a
`&[Rc<EffectType>]` in which you can access the effects like a slice.

For a complete example, please see [Title](/src/title.rs).

It is recommended that you use a `Provider`, React Context-like
approach as this library may drop `yewtil::store` in favour of the
upcoming Context API when Yew 0.19 comes.

## Licence

Copyright 2021 Kaede Hoshikawa

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
