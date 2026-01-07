use crate::scene::manager::Scene;
use crate::shaders::Shader;
use crate::ui::{inspector::Inspector, TextRenderer};
use std::rc::Rc;

pub struct UIManager {
    pub text_renderer: TextRenderer,
    pub ui_rect_shader: Rc<Shader>,
    pub inspector: Inspector,
}

impl UIManager {
    pub fn new(text_renderer: TextRenderer, ui_rect_shader: Rc<Shader>) -> Self {
        Self {
            text_renderer,
            ui_rect_shader,
            inspector: Inspector::new(1070.0, 500.0),
        }
    }

    pub fn render(&self, scene: &Scene, selected_object_id: Option<usize>) {
        // Inspector
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
