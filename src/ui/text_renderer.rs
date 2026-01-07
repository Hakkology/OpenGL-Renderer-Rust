extern crate gl;
use crate::shaders::{Shader, Texture};
use gl::types::*;
use glam::Mat4;
use rusttype::{point, Font, Scale};
use std::rc::Rc;

pub struct TextRenderer {
    shader: Rc<Shader>,
    vao: GLuint,
    vbo: GLuint,
    font: Font<'static>,
}

impl TextRenderer {
    pub fn new(shader: Rc<Shader>) -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            // 4 vertices, each with pos[2] and tex[2]
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (4 * 4 * std::mem::size_of::<f32>()) as isize,
                std::ptr::null(),
                gl::DYNAMIC_DRAW,
            );
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                4 * std::mem::size_of::<f32>() as i32,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                4 * std::mem::size_of::<f32>() as i32,
                (2 * std::mem::size_of::<f32>()) as *const _,
            );
            gl::BindVertexArray(0);
        }

        let font_data =
            std::fs::read("assets/fonts/DejaVuSans.ttf").expect("Could not find font file");
        let font = Font::try_from_vec(font_data).expect("Error constructing Font");

        Self {
            shader,
            vao,
            vbo,
            font,
        }
    }

    pub fn render_rect(
        &self,
        shader: &Shader,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: glam::Vec4,
        window_width: f32,
        window_height: f32,
    ) {
        let projection = Mat4::orthographic_rh_gl(0.0, window_width, 0.0, window_height, -1.0, 1.0);
        shader.use_program();
        shader.set_mat4("projection", &projection.to_cols_array());
        shader.set_vec4("u_Color", color.x, color.y, color.z, color.w);

        let x2 = x + width;
        let y2 = y + height;
        let vertices: [f32; 16] = [
            x, y2, 0.0, 0.0, x, y, 0.0, 1.0, x2, y2, 1.0, 0.0, x2, y, 1.0, 1.0,
        ];

        unsafe {
            gl::Disable(gl::DEPTH_TEST);
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const _,
            );
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
            gl::BindVertexArray(0);
            gl::Enable(gl::DEPTH_TEST);
        }
    }

    pub fn render_text(
        &self,
        text: &str,
        x: f32,
        y: f32,
        scale: f32,
        color: glam::Vec3,
        window_width: f32,
        window_height: f32,
    ) {
        let projection = Mat4::orthographic_rh_gl(0.0, window_width, 0.0, window_height, -1.0, 1.0);
        self.shader.use_program();
        self.shader
            .set_mat4("projection", &projection.to_cols_array());
        self.shader.set_vec3("u_Color", color.x, color.y, color.z);

        let scale = Scale::uniform(scale);
        let v_metrics = self.font.v_metrics(scale);
        let glyphs: Vec<_> = self
            .font
            .layout(text, scale, point(0.0, v_metrics.ascent))
            .collect();

        let width = glyphs
            .iter()
            .rev()
            .map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
            .next()
            .unwrap_or(0.0)
            .ceil() as u32;
        let height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;

        if width == 0 || height == 0 {
            return;
        }

        let mut pixel_data = vec![0u8; (width * height) as usize];

        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let px = x as i32 + bounding_box.min.x;
                    let py = y as i32 + bounding_box.min.y;
                    if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
                        pixel_data[(py as u32 * width + px as u32) as usize] = (v * 255.0) as u8;
                    }
                });
            }
        }

        let tex = Texture::new(width, height, &pixel_data, gl::RED);

        let x2 = x + width as f32;
        let y2 = y + height as f32;

        let vertices: [f32; 16] = [
            x, y2, 0.0, 0.0, // TL
            x, y, 0.0, 1.0, // BL
            x2, y2, 1.0, 0.0, // TR
            x2, y, 1.0, 1.0, // BR
        ];

        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Disable(gl::DEPTH_TEST);
            tex.bind(0);
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const _,
            );
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
            gl::BindVertexArray(0);
            gl::Enable(gl::DEPTH_TEST);
            gl::Disable(gl::BLEND);
        }
    }
}
