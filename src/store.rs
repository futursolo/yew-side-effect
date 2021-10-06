use std::collections::HashMap;
use std::rc::Rc;

use crate::utils::Id;
use crate::SideEffects;

pub(crate) enum Message<T: PartialEq + 'static> {
    Add((Id, Rc<T>)),
    Update((Id, Rc<T>)),
    Remove(Id),
}

#[derive(Debug, PartialEq)]
pub(crate) struct Store<T: PartialEq + 'static> {
    effect_ids: Vec<Id>,
    effects: HashMap<Id, Rc<T>>,
}

impl<T: PartialEq + 'static> Clone for Store<T> {
    fn clone(&self) -> Self {
        Self {
            effect_ids: self.effect_ids.clone(),
            effects: self.effects.clone(),
        }
    }
}

impl<T: PartialEq + 'static> Store<T> {
    pub fn new() -> Self {
        Self {
            effect_ids: Vec::new(),
            effects: HashMap::new(),
        }
    }

    pub fn reduce(&mut self, msg: Message<T>) {
        match msg {
            Message::Add((id, effect)) => self.add(id, effect),
            Message::Update((id, effect)) => self.update(id, effect),
            Message::Remove(id) => self.remove(id),
        }
    }

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

    pub fn has(&self, id: &Id) -> bool {
        self.effect_ids.contains(id)
    }

    pub fn get(&self) -> SideEffects<T> {
        let mut effects = Vec::new();

        for effect_id in self.effect_ids.iter() {
            if let Some(m) = self.effects.get(effect_id).cloned() {
                effects.push(m);
            }
        }

        SideEffects::new(effects)
    }
}
