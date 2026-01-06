use glfw::{Action, Key, MouseButton, WindowEvent};
use std::collections::HashMap;
use glam::Vec2;

pub struct Input {
    keys: HashMap<Key, Action>,
    mouse_buttons: HashMap<MouseButton, Action>,
    pub mouse_pos: Vec2,
    pub last_mouse_pos: Vec2,
    pub mouse_delta: Vec2,
    pub scroll_y: f32,
    first_mouse: bool,
}

impl Input {
    pub fn new() -> Self {
        Input {
            keys: HashMap::new(),
            mouse_buttons: HashMap::new(),
            mouse_pos: Vec2::ZERO,
            last_mouse_pos: Vec2::ZERO,
            mouse_delta: Vec2::ZERO,
            scroll_y: 0.0,
            first_mouse: true,
        }
    }

    pub fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::Key(key, _, action, _) => {
                self.keys.insert(*key, *action);
            }
            WindowEvent::MouseButton(btn, action, _) => {
                self.mouse_buttons.insert(*btn, *action);
            }
            WindowEvent::CursorPos(x, y) => {
                let current_pos = Vec2::new(*x as f32, *y as f32);
                
                if self.first_mouse {
                    self.last_mouse_pos = current_pos;
                    self.first_mouse = false;
                }

                self.mouse_delta = current_pos - self.last_mouse_pos;
                self.last_mouse_pos = current_pos;
                self.mouse_pos = current_pos;
            }
            WindowEvent::Scroll(_, y) => {
                self.scroll_y = *y as f32;
            }
            _ => {}
        }
    }

    pub fn reset_delta(&mut self) {
        self.mouse_delta = Vec2::ZERO;
        self.scroll_y = 0.0;
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
    
    pub fn is_mouse_button_pressed(&self, btn: MouseButton) -> bool {
        match self.mouse_buttons.get(&btn) {
            Some(Action::Press) => true,
            _ => false,
        }
    }
}
