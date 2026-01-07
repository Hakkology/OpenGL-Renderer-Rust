extern crate gl;
use gl::types::*;
use std::ptr;

pub struct Plane {
    vao: GLuint,
    vbo: GLuint,
    pub size: f32,
}

impl Plane {
    pub fn new(size: f32) -> Self {
        let mut plane = Plane {
            vao: 0,
            vbo: 0,
            size,
        };
        plane.init();
        plane
    }

    fn init(&mut self) {
        let half = self.size / 2.0;

        let vertices: [f32; 32] = [
            // positions          // tex coords   // normals
            -half, 0.0, half, 0.0, 0.0, 0.0, 1.0, 0.0, // Bottom-Left
            half, 0.0, half, 1.0, 0.0, 0.0, 1.0, 0.0, // Bottom-Right
            -half, 0.0, -half, 0.0, 1.0, 0.0, 1.0, 0.0, // Top-Left
            half, 0.0, -half, 1.0, 1.0, 0.0, 1.0, 0.0, // Top-Right
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

            let stride = (8 * std::mem::size_of::<GLfloat>()) as i32;

            // Position (0)
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);

            // TexCoord (2)
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (3 * std::mem::size_of::<GLfloat>()) as *const _,
            );
            gl::EnableVertexAttribArray(2);

            // Normal (1)
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (5 * std::mem::size_of::<GLfloat>()) as *const _,
            );
            gl::EnableVertexAttribArray(1);

            gl::BindVertexArray(0);
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for Plane {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}
