use glfw::{Action, Key};
use std::collections::HashMap;

pub struct Keyboard {
    keys: HashMap<Key, Action>,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }

    pub fn handle_key_event(&mut self, key: Key, action: Action) {
        self.keys.insert(key, action);
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        match self.keys.get(&key) {
            Some(Action::Press) | Some(Action::Repeat) => true,
            _ => false,
        }
    }

    pub fn is_key_just_pressed(&self, key: Key) -> bool {
        match self.keys.get(&key) {
            Some(Action::Press) => true,
            _ => false,
        }
    }
}
