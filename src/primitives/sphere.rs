extern crate gl;
use gl::types::*;
use std::ptr;
use std::f32::consts::PI;

pub struct Sphere {
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
    indices_count: i32,
    pub radius: f32,
}

impl Sphere {
    pub fn new(radius: f32, sectors: u32, stacks: u32) -> Self {
        let mut sphere = Sphere {
            vao: 0,
            vbo: 0,
            ebo: 0,
            indices_count: 0,
            radius,
        };
        sphere.init(sectors, stacks);
        sphere
    }

    fn init(&mut self, sectors: u32, stacks: u32) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let sector_step = 2.0 * PI / sectors as f32;
        let stack_step = PI / stacks as f32;

        for i in 0..=stacks {
            let stack_angle = PI / 2.0 - i as f32 * stack_step;
            let xy = self.radius * stack_angle.cos();
            let z = self.radius * stack_angle.sin();

            for j in 0..=sectors {
                let sector_angle = j as f32 * sector_step;

                let x = xy * sector_angle.cos();
                let y = xy * sector_angle.sin();
                
                // Position
                vertices.push(x);
                vertices.push(y);
                vertices.push(z);

                // Texture coords
                vertices.push(j as f32 / sectors as f32);
                vertices.push(i as f32 / stacks as f32);

                // Normals
                let nx = x / self.radius;
                let ny = y / self.radius;
                let nz = z / self.radius;
                vertices.push(nx);
                vertices.push(ny);
                vertices.push(nz);
            }
        }

        for i in 0..stacks {
            let mut k1 = i * (sectors + 1);
            let mut k2 = k1 + sectors + 1;

            for _ in 0..sectors {
                if i != 0 {
                    indices.push(k1);
                    indices.push(k2);
                    indices.push(k1 + 1);
                }

                if i != (stacks - 1) {
                    indices.push(k1 + 1);
                    indices.push(k2);
                    indices.push(k2 + 1);
                }
                k1 += 1;
                k2 += 1;
            }
        }

        self.indices_count = indices.len() as i32;

        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);
            gl::GenBuffers(1, &mut self.ebo);

            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<GLfloat>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<GLuint>()) as isize,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let stride = (8 * std::mem::size_of::<GLfloat>()) as i32;
            
            // Position
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);

            // TexCoord
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (3 * std::mem::size_of::<GLfloat>()) as *const _);
            gl::EnableVertexAttribArray(2);
             
            // Normal
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (5 * std::mem::size_of::<GLfloat>()) as *const _);
            gl::EnableVertexAttribArray(1);

            gl::BindVertexArray(0);
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, self.indices_count, gl::UNSIGNED_INT, ptr::null());
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for Sphere {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
        }
    }
}
