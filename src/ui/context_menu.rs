use crate::ui::{Button, TextRenderer};
use crate::shaders::Shader;
use glam::Vec4;

pub struct ContextMenu {
    pub is_visible: bool,
    pub x: f32,
    pub y: f32,
    pub target_id: Option<usize>, // The object this menu is valid for, if any
    
    // Dynamic buttons
    create_btn: Button,
    destroy_btn: Button,
}

impl ContextMenu {
    pub fn new() -> Self {
        Self {
            is_visible: false,
            x: 0.0,
            y: 0.0,
            target_id: None,
            create_btn: Button::new("Create Object", 0.0, 0.0, 150.0, 30.0),
            destroy_btn: Button::new("Destroy", 0.0, 0.0, 150.0, 30.0),
        }
    }

    pub fn show(&mut self, x: f32, y: f32, target_id: Option<usize>) {
        self.is_visible = true;
        self.x = x;
        // Invert Y for UI logic if needed, but usually we pass mouse coordinates directly
        // and handle projection in render. Our UI renderer is Ortho 0..Height.
        // If x,y comes from mouse (0 at top), we need to flip Y.
        // But let's assume the caller passes correct UI coordinates (0 at bottom).
        self.y = y; 
        self.target_id = target_id;
        
        let current_y = y;
        
        self.create_btn.x = x;
        self.create_btn.y = current_y - 30.0;
        
        if target_id.is_some() {
            self.destroy_btn.x = x;
            self.destroy_btn.y = current_y - 65.0; // 30 height + 5 space
        }
    }
    
    pub fn hide(&mut self) {
        self.is_visible = false;
        self.target_id = None;
    }
    
    pub fn draw(&self, renderer: &TextRenderer, rect_shader: &Shader, width: f32, height: f32) {
        if !self.is_visible { return; }
        
        // Draw Background
        let menu_w = 160.0;
        let menu_h = if self.target_id.is_some() { 75.0 } else { 40.0 };
        let bg_y = if self.target_id.is_some() { self.y - 70.0 } else { self.y - 35.0 };
        
        renderer.render_rect(
            rect_shader, 
            self.x - 5.0, 
            bg_y, 
            menu_w, 
            menu_h, 
            Vec4::new(0.2, 0.2, 0.2, 0.95), 
            width, 
            height
        );
        
        self.create_btn.draw(renderer, rect_shader, width, height);
        
        if self.target_id.is_some() {
            self.destroy_btn.draw(renderer, rect_shader, width, height);
        }
    }
    
    // Returns action: 0 = None, 1 = Create, 2 = Destroy
    pub fn check_clicks(&mut self, mx: f32, my: f32, height: f32) -> i32 {
        if !self.is_visible { return 0; }
        
        if self.create_btn.is_clicked(mx, my, height) {
            self.hide();
            return 1;
        }
        
        if self.target_id.is_some() && self.destroy_btn.is_clicked(mx, my, height) {
            self.hide();
            return 2;
        }
        
        0
    }
}
