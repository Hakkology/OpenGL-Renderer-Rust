#![allow(dead_code)]
extern crate gl;

use std::rc::Rc;
use std::ptr;
use gl::types::{GLuint, GLfloat};

use crate::shaders::Shader;
use crate::math::Vector2D;
use super::Shape;

#[allow(dead_code)]
pub struct Triangle {
    vao: GLuint,
    vbo: GLuint,
    shader: Rc<Shader>,
    vertices: [Vector2D; 3],
    normals: [Vector2D; 3],
}

impl Triangle {
    // Yeni bir üçgen oluşturur
    pub fn new(shader: Rc<Shader>, v1: Vector2D, v2: Vector2D, v3: Vector2D) -> Triangle {
        let edge1 = Vector2D::new(v2.x - v1.x, v2.y - v1.y);
        // Basit bir normal hesaplama (2D için Z ekseni dikkate alınarak)
        let normal = Vector2D::new(edge1.y, -edge1.x).normalize();

        let mut triangle = Triangle { 
            vao: 0, 
            vbo: 0, 
            shader,
            vertices: [v1, v2, v3],
            normals: [normal, normal, normal],
        };
        triangle.init();
        triangle
    }

    // Shader'ı değiştirmek için metot
    pub fn set_shader(&mut self, shader: Rc<Shader>) {
        self.shader = shader;
    }
}

impl Shape for Triangle {
    // Üçgeni başlatır ve OpenGL'e yükler
    fn init(&mut self) {
        let mut vertices: Vec<GLfloat> = Vec::new();
        for i in 0..3 {
            vertices.extend_from_slice(&[
                self.vertices[i].x, self.vertices[i].y, 0.0,
                self.normals[i].x, self.normals[i].y, 1.0,
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

            // Position attribute
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 6 * std::mem::size_of::<GLfloat>() as i32, ptr::null());
            gl::EnableVertexAttribArray(0);

            // Normal/Color attribute
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 6 * std::mem::size_of::<GLfloat>() as i32, (3 * std::mem::size_of::<GLfloat>()) as *const _);
            gl::EnableVertexAttribArray(1);

            gl::BindVertexArray(0);
        }
    }

    // Üçgeni çizer
    fn draw(&self) {
        unsafe {
            self.shader.use_program();
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::BindVertexArray(0);
        }
    }
}
