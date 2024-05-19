use std::collections::HashMap;

use glfw::Key;

use crate::utils::Vector2D;

pub struct EventHandler {
    pub keys_pressed: HashMap<Key, usize>,

    pub mouse_pos: Vector2D,

    pub scroll: Vector2D,

    pub width: f32,
    pub height: f32,

    pub lmb: bool,
    pub rmb: bool,
}

impl EventHandler {
    pub fn new() -> Self {
        Self { 
            keys_pressed: HashMap::new(),
            mouse_pos: Vector2D::new(1.0, 1.0),
            width: 1.0,
            height: 1.0,

            scroll: Vector2D::ZERO,

            rmb: false,
            lmb: false,
        }
    }

    pub fn on_key_press(&mut self, key: Key) {
        let key_handle = self.keys_pressed.len();
        self.keys_pressed.insert(key, key_handle);
    }

    pub fn on_key_release(&mut self, key: Key) {
        self.keys_pressed.remove(&key);
    }

    pub fn on_mouse_move(&mut self, x: f64, y: f64) {
        self.mouse_pos.x = x as f32;
        self.mouse_pos.y = y as f32;
    }

    pub fn on_lmb_press(&mut self) {
        self.lmb = true;
    } 
    pub fn on_lmb_release(&mut self) {
        self.lmb = false;
    } 

    pub fn on_rmb_press(&mut self) {
        self.rmb = true;
    } 
    pub fn on_rmb_release(&mut self) {
        self.rmb = false;
    } 

    pub fn on_scroll_change(&mut self, change: Vector2D){
        self.scroll = change;
    }

    pub fn on_window_resize(&mut self, w: i32, h: i32) {
        self.width = w as f32;
        self.height = h as f32;
    }
}
