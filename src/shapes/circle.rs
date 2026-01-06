#![allow(dead_code)]
extern crate gl;

use std::rc::Rc;
use std::ptr;
use std::f32::consts::PI;
use gl::types::{GLuint, GLfloat};

use crate::shaders::Shader;
use crate::math::Vector2D;
use super::Shape;

#[allow(dead_code)]
pub struct Circle {
    vao: GLuint,
    vbo: GLuint,
    shader: Rc<Shader>,
    center: Vector2D,
    radius: f32,
    segments: usize,
}

impl Circle {
    pub fn new(shader: Rc<Shader>, center: Vector2D, radius: f32, segments: usize) -> Circle {
        let mut circle = Circle {
            vao: 0,
            vbo: 0,
            shader,
            center,
            radius,
            segments: if segments < 3 { 3 } else { segments },
        };
        circle.init();
        circle
    }
}

impl Shape for Circle {
    fn init(&mut self) {
        let mut vertices: Vec<GLfloat> = Vec::new();
        
        // Center vertex
        vertices.extend_from_slice(&[
            self.center.x, self.center.y, 0.0,
            0.0, 0.0, 1.0 // Normal
        ]);

        for i in 0..=self.segments {
            let theta = 2.0 * PI * (i as f32) / (self.segments as f32);
            let x = self.center.x + self.radius * theta.cos();
            let y = self.center.y + self.radius * theta.sin();

            vertices.extend_from_slice(&[
                x, y, 0.0,
                0.0, 0.0, 1.0
            ]);
        }

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
            // segments + 1 points around + 1 center = segments + 2 vertices
            // DrawArrays count is number of vertices.
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, (self.segments + 2) as i32);
            gl::BindVertexArray(0);
        }
    }
}
