use crate::scene::manager::Scene;
use crate::shaders::Shader;
use crate::ui::{inspector::Inspector, Button, TextRenderer};
use glam::Vec3;
use std::rc::Rc;

pub struct UIManager {
    pub text_renderer: TextRenderer,
    pub ui_rect_shader: Rc<Shader>,
    pub pause_button: Button,
    pub inspector: Inspector,
}

impl UIManager {
    pub fn new(text_renderer: TextRenderer, ui_rect_shader: Rc<Shader>) -> Self {
        Self {
            text_renderer,
            ui_rect_shader,
            pause_button: Button::new("Pause", 1170.0, 660.0, 100.0, 40.0),
            inspector: Inspector::new(1070.0, 500.0),
        }
    }

    pub fn render(&self, scene: &Scene, selected_object_id: Option<usize>, is_paused: bool) {
        // 1. Top Panel
        self.text_renderer.render_rect(
            &self.ui_rect_shader,
            10.0,
            660.0,
            180.0,
            50.0,
            glam::Vec4::new(0.0, 0.0, 0.0, 0.5),
            1280.0,
            720.0,
        );
        self.text_renderer.render_text(
            "Hakkology",
            20.0,
            665.0,
            32.0,
            Vec3::new(1.0, 1.0, 1.0),
            1280.0,
            720.0,
        );

        // 2. Pause Button
        let mut pause_btn = self.pause_button.clone();
        if is_paused {
            pause_btn.text = "Resume".to_string();
        }
        pause_btn.draw(&self.text_renderer, &self.ui_rect_shader, 1280.0, 720.0);

        // 3. Inspector
        if let Some(id) = selected_object_id {
            if let Some(obj) = scene.objects.iter().find(|o| o.id == id) {
                self.inspector.draw(
                    &self.text_renderer,
                    &self.ui_rect_shader,
                    1280.0,
                    720.0,
                    &obj.name,
                    obj.transform.position,
                );
            }
        }
    }
}
