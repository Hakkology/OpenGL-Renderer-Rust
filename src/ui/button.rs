use crate::shaders::Shader;
use crate::ui::TextRenderer;
use glam::{Vec3, Vec4};

#[derive(Clone)]
pub struct Button {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub bg_color: Vec4,
    pub text_color: Vec3,
}

impl Button {
    pub fn new(text: &str, x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            text: text.to_string(),
            x,
            y,
            width,
            height,
            bg_color: Vec4::new(0.2, 0.2, 0.2, 0.8),
            text_color: Vec3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn is_clicked(&self, mouse_x: f32, mouse_y: f32, window_height: f32) -> bool {
        // OpenGL coords usually 0 at bottom, but GLFW mouse often 0 at top.
        // If window is 800x600, mouse (x, 0) is top.
        // My UI rendering uses 0 at bottom (Ortho 0..height).
        // So we need to flip mouse_y.
        let corrected_mouse_y = window_height - mouse_y;

        mouse_x >= self.x
            && mouse_x <= self.x + self.width
            && corrected_mouse_y >= self.y
            && corrected_mouse_y <= self.y + self.height
    }

    pub fn draw(
        &self,
        renderer: &TextRenderer,
        rect_shader: &Shader,
        window_width: f32,
        window_height: f32,
    ) {
        // Draw background
        renderer.render_rect(
            rect_shader,
            self.x,
            self.y,
            self.width,
            self.height,
            self.bg_color,
            window_width,
            window_height,
        );

        // Draw text centered roughly
        let text_scale = self.height * 0.6;
        let text_x = self.x + 10.0;
        let text_y = self.y + (self.height - text_scale) / 2.0 + 5.0;

        renderer.render_text(
            &self.text,
            text_x,
            text_y,
            text_scale,
            self.text_color,
            window_width,
            window_height,
        );
    }
}
