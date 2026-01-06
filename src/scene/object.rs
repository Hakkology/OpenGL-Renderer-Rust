use crate::scene::transform::{Transform, Transform2D};
use crate::shaders::Shader;
use crate::primitives::{Cube, Sphere, Capsule};
// use crate::shapes::{Rectangle, Circle, Triangle}; // Unused/Incompatible shapes for now

use std::rc::Rc;

pub trait Renderable {
    fn draw(&self);
}

// 3D Primitive Implementation
impl Renderable for Cube {
    fn draw(&self) { self.draw(); }
}
impl Renderable for Sphere {
    fn draw(&self) { self.draw(); }
}
impl Renderable for Capsule {
    fn draw(&self) { self.draw(); }
}

impl<T: Renderable + ?Sized> Renderable for Rc<T> {
    fn draw(&self) {
        (**self).draw();
    }
}

impl<T: Renderable + ?Sized> Renderable for Box<T> {
    fn draw(&self) {
        (**self).draw();
    }
}

use crate::scene::material::Material;

pub struct SceneObject3D<T: Renderable> {
    pub transform: Transform,
    pub renderable: T,
    pub material: Rc<dyn Material>,
}

use crate::scene::context::RenderContext;

impl<T: Renderable> SceneObject3D<T> {
    pub fn new(renderable: T, material: Rc<dyn Material>) -> Self {
        Self {
            transform: Transform::default(),
            renderable,
            material,
        }
    }

    pub fn render(&self, ctx: &RenderContext) {
        self.material.apply();
        let shader = self.material.shader();
        
        // Matrices
        shader.set_mat4("projection", &ctx.projection.to_cols_array());
        shader.set_mat4("view", &ctx.view.to_cols_array());
        shader.set_mat4("model", &self.transform.to_matrix().to_cols_array());

        // Toggles
        shader.set_int("u_UseLighting", if self.material.is_lit() { 1 } else { 0 });
        shader.set_int("u_UseShadows", if self.material.receive_shadows() { 1 } else { 0 });
        
        // Lighting
        if self.material.is_lit() {
            ctx.apply_lighting(shader);
        }

        self.renderable.draw();
    }
    
    pub fn render_depth(&self, shader: &Shader) {
        shader.set_mat4("model", &self.transform.to_matrix().to_cols_array());
        self.renderable.draw();
    }
}

pub struct SceneObject2D<T: Renderable> {
    pub transform: Transform2D,
    pub renderable: T,
}

impl<T: Renderable> SceneObject2D<T> {
    pub fn new(renderable: T) -> Self {
        Self {
            transform: Transform2D::default(),
            renderable,
        }
    }

    pub fn draw(&self, shader: &Shader) {
        shader.set_mat4("model", &self.transform.to_matrix().to_cols_array());
        self.renderable.draw();
    }
}
