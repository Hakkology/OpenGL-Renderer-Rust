use glfw::{Action, MouseButton};
use std::collections::HashMap;
use glam::Vec2;

pub struct Mouse {
    buttons: HashMap<MouseButton, Action>,
    pub pos: Vec2,
    pub last_pos: Vec2,
    pub delta: Vec2,
    pub scroll_y: f32,
    first_mouse: bool,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            buttons: HashMap::new(),
            pos: Vec2::ZERO,
            last_pos: Vec2::ZERO,
            delta: Vec2::ZERO,
            scroll_y: 0.0,
            first_mouse: true,
        }
    }

    pub fn handle_button_event(&mut self, button: MouseButton, action: Action) {
        self.buttons.insert(button, action);
    }

    pub fn handle_move_event(&mut self, x: f32, y: f32) {
        let current_pos = Vec2::new(x, y);
        
        if self.first_mouse {
            self.last_pos = current_pos;
            self.first_mouse = false;
        }

        self.delta += current_pos - self.last_pos;
        self.last_pos = current_pos;
        self.pos = current_pos;
    }

    pub fn handle_scroll_event(&mut self, _x: f32, y: f32) {
        self.scroll_y += y;
    }

    pub fn reset_delta(&mut self) {
        self.delta = Vec2::ZERO;
        self.scroll_y = 0.0;
    }

    pub fn is_button_pressed(&self, btn: MouseButton) -> bool {
        match self.buttons.get(&btn) {
            Some(Action::Press) => true,
            _ => false,
        }
    }
}
