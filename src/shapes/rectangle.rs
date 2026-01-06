extern crate gl;

use std::rc::Rc;
use std::ptr;
use gl::types::{GLuint, GLfloat};

use crate::shaders::Shader;
use crate::math::Vector2D;
use super::Shape;

pub struct Rectangle {
    vao: GLuint,
    vbo: GLuint,
    shader: Rc<Shader>,
    top_left: Vector2D,
    width: f32,
    height: f32,
}

impl Rectangle {
    pub fn new(shader: Rc<Shader>, top_left: Vector2D, width: f32, height: f32) -> Rectangle {
        let mut rect = Rectangle {
            vao: 0,
            vbo: 0,
            shader,
            top_left,
            width,
            height,
        };
        rect.init();
        rect
    }
}

impl Shape for Rectangle {
    fn init(&mut self) {
        let x = self.top_left.x;
        let y = self.top_left.y;
        let w = self.width;
        let h = self.height;

        // GL_TRIANGLE_STRIP order:
        // 1--3
        // | /|
        // |/ |
        // 0--2 (but usually we define TL, BL, TR, BR or similar)
        // Let's do:
        // V0: TL (x, y)
        // V1: BL (x, y - h)
        // V2: TR (x + w, y)
        // V3: BR (x + w, y - h)
        
        let vertices: [f32; 24] = [
            // Pos (x, y, z)       // Color/Normal (dummy)
            x,     y,     0.0,     0.0, 0.0, 1.0, // TL
            x,     y - h, 0.0,     0.0, 0.0, 1.0, // BL
            x + w, y,     0.0,     0.0, 0.0, 1.0, // TR
            x + w, y - h, 0.0,     0.0, 0.0, 1.0, // BR
        ];

        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);

            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<GLfloat>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Position
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 6 * std::mem::size_of::<GLfloat>() as i32, ptr::null());
            gl::EnableVertexAttribArray(0);

            // Normal/Color
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 6 * std::mem::size_of::<GLfloat>() as i32, (3 * std::mem::size_of::<GLfloat>()) as *const _);
            gl::EnableVertexAttribArray(1);

            gl::BindVertexArray(0);
        }
    }

    fn draw(&self) {
        unsafe {
            self.shader.use_program();
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
            gl::BindVertexArray(0);
        }
    }
}
