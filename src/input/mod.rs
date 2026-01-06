pub mod keyboard;
pub mod mouse;

use glfw::{Key, MouseButton, WindowEvent};
use glam::Vec2;
pub use keyboard::Keyboard;
pub use mouse::Mouse;

pub struct Input {
    pub keyboard: Keyboard,
    pub mouse: Mouse,
}

impl Input {
    pub fn new() -> Self {
        Self {
            keyboard: Keyboard::new(),
            mouse: Mouse::new(),
        }
    }

    pub fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::Key(key, _, action, _) => {
                self.keyboard.handle_key_event(*key, *action);
            }
            WindowEvent::MouseButton(btn, action, _) => {
                self.mouse.handle_button_event(*btn, *action);
            }
            WindowEvent::CursorPos(x, y) => {
                self.mouse.handle_move_event(*x as f32, *y as f32);
            }
            WindowEvent::Scroll(x, y) => {
                self.mouse.handle_scroll_event(*x as f32, *y as f32);
            }
            _ => {}
        }
    }

    // Helper methods to keep backward compatibility or provide easy access
    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.keyboard.is_key_pressed(key)
    }

    pub fn is_key_just_pressed(&self, key: Key) -> bool {
        self.keyboard.is_key_just_pressed(key)
    }

    pub fn is_mouse_button_pressed(&self, btn: MouseButton) -> bool {
        self.mouse.is_button_pressed(btn)
    }

    pub fn reset_delta(&mut self) {
        self.mouse.reset_delta();
    }

    // Direct access to common fields
    pub fn mouse_pos(&self) -> Vec2 {
        self.mouse.pos
    }

    pub fn mouse_delta(&self) -> Vec2 {
        self.mouse.delta
    }
}
