use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use crate::object::Object;

#[derive(Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer_env: Option<Rc<RefCell<Environment>>>,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            outer_env: None,
        }
    }

    pub fn new_enclosed_env(outer_env: Rc<RefCell<Environment>>) -> Self {
        Self {
            store: HashMap::new(),
            outer_env: Some(outer_env),
        }
    }

    pub fn upsert(&mut self, name: String, value: Object) {
        self.store.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.store.get(name) {
            Some(obj) => Some(obj.clone()),
            None => match &self.outer_env {
                Some(outer_env) => outer_env.borrow().get(name),
                None => None,
            },
        }
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut a = String::new();
        for (k, _v) in self.store.iter() {
            a.push_str(k);
        }
        write!(f, "{}", a)
    }
}
