use crate::object::Object;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Default)]
pub struct Env<'a> {
    parent: Option<&'a Env<'a>>,
    vars: HashMap<String, Object>,
}

impl<'a> Env<'a> {
    pub fn new() -> Box<Env<'a>> {
        Box::new(Env {
            vars: HashMap::new(),
            parent: None,
        })
    }

    pub fn extend(parent: &'a Self) -> Box<Env<'a>> {
        Box::new(Env {
            vars: HashMap::new(),
            parent: Some(parent),
        })
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.vars.get(name) {
            Some(value) => Some(value.clone()),
            None => self.parent.and_then(|parent| parent.get(name)),
        }
    }

    pub fn set(&mut self, name: &str, val: Object) {
        self.vars.insert(name.to_string(), val);
    }
}
