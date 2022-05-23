use crate::object::Object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Default)]
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    vars: HashMap<String, Object>,
}

impl Env {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn extend(parent: Rc<RefCell<Self>>) -> Env {
        let mut vars = HashMap::new();
        vars.extend(parent.borrow().vars.iter().map(|(k, v)| (k.clone(), v.clone())));

        Env {
            vars: vars,
            parent: None,
        }
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.vars.get(name) {
            Some(value) => Some(value.clone()),
            None => self
                .parent
                .as_ref()
                .and_then(|o| o.borrow().get(name).clone()),
        }
    }

    pub fn set(&mut self, name: &str, val: Object) {
        self.vars.insert(name.to_string(), val);
    }
}
