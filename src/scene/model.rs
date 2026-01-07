extern crate gl;
use gl::types::*;
use std::rc::Rc;
use crate::scene::object::Renderable;

pub struct Mesh {
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
    indices_count: i32,
}

impl Mesh {
    pub fn new(vertices: &[f32], indices: &[u32]) -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as isize,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let stride = (8 * std::mem::size_of::<f32>()) as i32;
            
            // Position
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);
            
            // Texture Coords
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (3 * std::mem::size_of::<f32>()) as *const _);
            gl::EnableVertexAttribArray(2);
            
            // Normals
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (5 * std::mem::size_of::<f32>()) as *const _);
            gl::EnableVertexAttribArray(1);

            gl::BindVertexArray(0);
        }

        Self {
            vao,
            vbo,
            ebo,
            indices_count: indices.len() as i32,
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, self.indices_count, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
        }
    }
}

pub struct Model {
    pub meshes: Vec<Mesh>,
}

impl Model {
    pub fn new(meshes: Vec<Mesh>) -> Self {
        Self { meshes }
    }
}

impl Renderable for Model {
    fn draw(&self) {
        for mesh in &self.meshes {
            mesh.draw();
        }
    }
}


