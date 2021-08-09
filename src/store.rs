use std::collections::HashMap;
use std::rc::Rc;

use yew::agent::AgentLink;
use yewtil::store::{Store, StoreWrapper};

use crate::effects::Effects;
use crate::utils::Id;

pub enum Message<T: Default + PartialEq + 'static> {
    Add((Id, Rc<T>)),
    Update((Id, Rc<T>)),
    Remove(Id),
}

#[derive(Default)]
pub struct EffectStore<T: Default + PartialEq + 'static> {
    effect_ids: Vec<Id>,
    pub effects: HashMap<Id, Rc<T>>,
}

impl<T: Default + PartialEq + 'static> Store for EffectStore<T> {
    type Action = Message<T>;
    type Input = Message<T>;

    fn new() -> EffectStore<T> {
        EffectStore::default()
    }

    fn handle_input(&self, link: AgentLink<StoreWrapper<Self>>, msg: Self::Input) {
        link.send_message(msg);
    }

    fn reduce(&mut self, msg: Self::Action) {
        match msg {
            Message::Add((id, effect)) => self.add_effect(id, effect),
            Message::Update((id, effect)) => self.update_effect(id, effect),
            Message::Remove(id) => self.remove_effect(id),
        }
    }
}

impl<T: Default + PartialEq + 'static> EffectStore<T> {
    pub fn add_effect(&mut self, id: Id, effect: Rc<T>) {
        self.effect_ids.push(id.clone());
        self.effects.insert(id, effect);
    }

    pub fn update_effect(&mut self, id: Id, effect: Rc<T>) {
        self.effects.insert(id, effect);
    }

    pub fn remove_effect(&mut self, id: Id) {
        self.effect_ids.retain(|m| m != &id);
        self.effects.remove(&id);
    }

    pub fn get_effects(&self) -> Effects<T> {
        let mut effects = Vec::new();

        for effect_id in self.effect_ids.iter() {
            if let Some(m) = self.effects.get(effect_id).cloned() {
                effects.push(m);
            }
        }

        Effects::new(effects)
    }
}
