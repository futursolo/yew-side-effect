use std::collections::HashMap;
use std::rc::Rc;

use yew::agent::AgentLink;
use yewtil::store::StoreWrapper;

use crate::utils::Id;
use crate::SideEffects;

pub enum Message<T: 'static> {
    Add((Id, Rc<T>)),
    Update((Id, Rc<T>)),
    Remove(Id),
}

pub struct Store<T: 'static> {
    effect_ids: Vec<Id>,
    effects: HashMap<Id, Rc<T>>,
}

impl<T: 'static> yewtil::store::Store for Store<T> {
    type Action = Message<T>;
    type Input = Message<T>;

    fn new() -> Self {
        Self {
            effect_ids: Vec::new(),
            effects: HashMap::new(),
        }
    }

    fn handle_input(&self, link: AgentLink<StoreWrapper<Self>>, msg: Self::Input) {
        link.send_message(msg);
    }

    fn reduce(&mut self, msg: Self::Action) {
        match msg {
            Message::Add((id, effect)) => self.add(id, effect),
            Message::Update((id, effect)) => self.update(id, effect),
            Message::Remove(id) => self.remove(id),
        }
    }
}

impl<T: 'static> Store<T> {
    fn add(&mut self, id: Id, effect: Rc<T>) {
        self.effect_ids.push(id.clone());
        self.effects.insert(id, effect);
    }

    fn update(&mut self, id: Id, effect: Rc<T>) {
        self.effects.insert(id, effect);
    }

    fn remove(&mut self, id: Id) {
        self.effect_ids.retain(|m| m != &id);
        self.effects.remove(&id);
    }

    pub(crate) fn get(&self) -> SideEffects<T> {
        let mut effects = Vec::new();

        for effect_id in self.effect_ids.iter() {
            if let Some(m) = self.effects.get(effect_id).cloned() {
                effects.push(m);
            }
        }

        SideEffects::new(effects)
    }
}
