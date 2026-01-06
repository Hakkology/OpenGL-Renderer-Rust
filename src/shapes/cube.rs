extern crate gl;

use std::rc::Rc;
use std::ptr;
use gl::types::{GLuint, GLfloat};

use crate::shaders::Shader;
use super::Shape;

pub struct Cube {
    vao: GLuint,
    vbo: GLuint,
    shader: Rc<Shader>,
}

impl Cube {
    pub fn new(shader: Rc<Shader>) -> Cube {
        let mut cube = Cube {
            vao: 0,
            vbo: 0,
            shader,
        };
        cube.init();
        cube
    }
}

impl Shape for Cube {
    fn init(&mut self) {
        let vertices: [f32; 288] = [
            // positions          // texture coords // normals (simplified, not exact)
            -0.5, -0.5, -0.5,  0.0, 0.0, 0.0, 0.0, -1.0, 
             0.5, -0.5, -0.5,  1.0, 0.0, 0.0, 0.0, -1.0,
             0.5,  0.5, -0.5,  1.0, 1.0, 0.0, 0.0, -1.0,
             0.5,  0.5, -0.5,  1.0, 1.0, 0.0, 0.0, -1.0,
            -0.5,  0.5, -0.5,  0.0, 1.0, 0.0, 0.0, -1.0,
            -0.5, -0.5, -0.5,  0.0, 0.0, 0.0, 0.0, -1.0,
    
            -0.5, -0.5,  0.5,  0.0, 0.0, 0.0, 0.0, 1.0,
             0.5, -0.5,  0.5,  1.0, 0.0, 0.0, 0.0, 1.0,
             0.5,  0.5,  0.5,  1.0, 1.0, 0.0, 0.0, 1.0,
             0.5,  0.5,  0.5,  1.0, 1.0, 0.0, 0.0, 1.0,
            -0.5,  0.5,  0.5,  0.0, 1.0, 0.0, 0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0, 0.0, 0.0, 0.0, 1.0,
    
            -0.5,  0.5,  0.5,  1.0, 0.0, -1.0, 0.0, 0.0,
            -0.5,  0.5, -0.5,  1.0, 1.0, -1.0, 0.0, 0.0,
            -0.5, -0.5, -0.5,  0.0, 1.0, -1.0, 0.0, 0.0,
            -0.5, -0.5, -0.5,  0.0, 1.0, -1.0, 0.0, 0.0,
            -0.5, -0.5,  0.5,  0.0, 0.0, -1.0, 0.0, 0.0,
            -0.5,  0.5,  0.5,  1.0, 0.0, -1.0, 0.0, 0.0,
    
             0.5,  0.5,  0.5,  1.0, 0.0, 1.0, 0.0, 0.0,
             0.5,  0.5, -0.5,  1.0, 1.0, 1.0, 0.0, 0.0,
             0.5, -0.5, -0.5,  0.0, 1.0, 1.0, 0.0, 0.0,
             0.5, -0.5, -0.5,  0.0, 1.0, 1.0, 0.0, 0.0,
             0.5, -0.5,  0.5,  0.0, 0.0, 1.0, 0.0, 0.0,
             0.5,  0.5,  0.5,  1.0, 0.0, 1.0, 0.0, 0.0,
    
            -0.5, -0.5, -0.5,  0.0, 1.0, 0.0, -1.0, 0.0,
             0.5, -0.5, -0.5,  1.0, 1.0, 0.0, -1.0, 0.0,
             0.5, -0.5,  0.5,  1.0, 0.0, 0.0, -1.0, 0.0,
             0.5, -0.5,  0.5,  1.0, 0.0, 0.0, -1.0, 0.0,
            -0.5, -0.5,  0.5,  0.0, 0.0, 0.0, -1.0, 0.0,
            -0.5, -0.5, -0.5,  0.0, 1.0, 0.0, -1.0, 0.0,
    
            -0.5,  0.5, -0.5,  0.0, 1.0, 0.0, 1.0, 0.0,
             0.5,  0.5, -0.5,  1.0, 1.0, 0.0, 1.0, 0.0,
             0.5,  0.5,  0.5,  1.0, 0.0, 0.0, 1.0, 0.0,
             0.5,  0.5,  0.5,  1.0, 0.0, 0.0, 1.0, 0.0,
            -0.5,  0.5,  0.5,  0.0, 0.0, 0.0, 1.0, 0.0,
            -0.5,  0.5, -0.5,  0.0, 1.0, 0.0, 1.0, 0.0
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

            // Position (3 floats)
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 8 * std::mem::size_of::<GLfloat>() as i32, ptr::null());
            gl::EnableVertexAttribArray(0);

            // TexCoord (2 floats)
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 8 * std::mem::size_of::<GLfloat>() as i32, (3 * std::mem::size_of::<GLfloat>()) as *const _);
            gl::EnableVertexAttribArray(2);
             
             // Normal (3 floats) - Not used in shader yet but good to have
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 8 * std::mem::size_of::<GLfloat>() as i32, (5 * std::mem::size_of::<GLfloat>()) as *const _);
            gl::EnableVertexAttribArray(1);

            gl::BindVertexArray(0);
        }
    }

    fn draw(&self) {
        unsafe {
            self.shader.use_program();
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            gl::BindVertexArray(0);
        }
    }
}
