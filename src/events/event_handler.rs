use std::collections::HashMap;

use glfw::Key;

pub struct EventHandler {
    pub keys_pressed: HashMap<Key, usize>,

}

impl EventHandler {
    pub fn new() -> Self {
        Self { keys_pressed: HashMap::new() }
    }

    pub fn on_key_press(&mut self, key: Key) {
        let key_handle = self.keys_pressed.len();
        self.keys_pressed.insert(key, key_handle);
    }

    pub fn on_key_release(&mut self, key: Key) {
        self.keys_pressed.remove(&key);
    }

    pub fn on_mouse_move(&mut self, ) {

    }
}
