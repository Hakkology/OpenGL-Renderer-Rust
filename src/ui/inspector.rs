use crate::config::ui as ui_cfg;
use crate::shaders::Shader;
use crate::ui::{Button, TextRenderer};
use glam::Vec3;

pub struct Inspector {
    pub x_inc: Button,
    pub x_dec: Button,
    pub y_inc: Button,
    pub y_dec: Button,
    pub z_inc: Button,
    pub z_dec: Button,
    x: f32,
    y: f32,
}

impl Inspector {
    pub fn new(x: f32, y: f32) -> Self {
        let btn_w = 30.0;
        let btn_h = 30.0;
        let spacing = 5.0;
        let row_h = 40.0;

        let _start_y = y; // This will be bottom of panel
                          // Offsets from bottom (y)
        let row3_y = y + 10.0; // Z
        let row2_y = row3_y + row_h; // Y
        let row1_y = row2_y + row_h; // X

        let label_w = 110.0; // Space for "X: -0.0"

        Self {
            x,
            y,
            x_dec: Button::new("<", x + label_w, row1_y, btn_w, btn_h),
            x_inc: Button::new(">", x + label_w + btn_w + spacing, row1_y, btn_w, btn_h),

            y_dec: Button::new("<", x + label_w, row2_y, btn_w, btn_h),
            y_inc: Button::new(">", x + label_w + btn_w + spacing, row2_y, btn_w, btn_h),

            z_dec: Button::new("<", x + label_w, row3_y, btn_w, btn_h),
            z_inc: Button::new(">", x + label_w + btn_w + spacing, row3_y, btn_w, btn_h),
        }
    }

    pub fn draw(
        &self,
        renderer: &TextRenderer,
        rect_shader: &Shader,
        width: f32,
        height: f32,
        selected_name: &str,
        current_pos: Vec3,
    ) {
        let panel_h = 160.0;
        let panel_w = 200.0;

        // Draw Main Background (Darker, rounded look simulated by color)
        renderer.render_rect(
            rect_shader,
            self.x - 10.0,
            self.y,
            panel_w,
            panel_h,
            glam::Vec4::new(0.1, 0.1, 0.15, 0.9),
            width,
            height,
        );

        // Draw Header Strip
        renderer.render_rect(
            rect_shader,
            self.x - 10.0,
            self.y + panel_h - 35.0,
            panel_w,
            35.0,
            glam::Vec4::new(0.2, 0.3, 0.4, 0.9),
            width,
            height,
        );

        // Header Text
        renderer.render_text(
            selected_name,
            self.x,
            self.y + panel_h - 28.0,
            20.0,
            Vec3::ONE,
            width,
            height,
        );

        // Labels with values
        let val_scale = ui_cfg::FONT_SIZE;
        let label_x = self.x;

        renderer.render_text(
            &format!("X: {:.2}", current_pos.x),
            label_x,
            self.x_dec.y + 8.0,
            val_scale,
            Vec3::new(1.0, 0.5, 0.5),
            width,
            height,
        );
        self.x_dec.draw(renderer, rect_shader, width, height);
        self.x_inc.draw(renderer, rect_shader, width, height);

        renderer.render_text(
            &format!("Y: {:.2}", current_pos.y),
            label_x,
            self.y_dec.y + 8.0,
            val_scale,
            Vec3::new(0.5, 1.0, 0.5),
            width,
            height,
        );
        self.y_dec.draw(renderer, rect_shader, width, height);
        self.y_inc.draw(renderer, rect_shader, width, height);

        renderer.render_text(
            &format!("Z: {:.2}", current_pos.z),
            label_x,
            self.z_dec.y + 8.0,
            val_scale,
            Vec3::new(0.5, 0.5, 1.0),
            width,
            height,
        );
        self.z_dec.draw(renderer, rect_shader, width, height);
        self.z_inc.draw(renderer, rect_shader, width, height);
    }

    pub fn check_clicks(&self, mx: f32, my: f32, height: f32) -> Vec3 {
        let mut delta = Vec3::ZERO;
        let step = 0.5;

        if self.x_dec.is_clicked(mx, my, height) {
            delta.x -= step;
        }
        if self.x_inc.is_clicked(mx, my, height) {
            delta.x += step;
        }

        if self.y_dec.is_clicked(mx, my, height) {
            delta.y -= step;
        }
        if self.y_inc.is_clicked(mx, my, height) {
            delta.y += step;
        }

        if self.z_dec.is_clicked(mx, my, height) {
            delta.z -= step;
        }
        if self.z_inc.is_clicked(mx, my, height) {
            delta.z += step;
        }

        delta
    }
}
