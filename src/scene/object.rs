use crate::primitives::{Capsule, Cube, Plane, Sphere};
use crate::scene::collider::Collider;
use crate::scene::transform::{Transform, Transform2D};
use crate::shaders::Shader;
// use crate::shapes::{Rectangle, Circle, Triangle}; // Unused/Incompatible shapes for now

use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

pub trait Renderable {
    fn draw(&self);
}

// 3D Primitive Implementation
impl Renderable for Cube {
    fn draw(&self) {
        self.draw();
    }
}
impl Renderable for Sphere {
    fn draw(&self) {
        self.draw();
    }
}
impl Renderable for Capsule {
    fn draw(&self) {
        self.draw();
    }
}
impl Renderable for Plane {
    fn draw(&self) {
        self.draw();
    }
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

use crate::logic::Controller;
use crate::scene::material::Material;

pub struct SceneObject3D {
    pub id: usize,
    pub name: String,
    pub transform: Transform,
    pub renderable: Box<dyn Renderable>,
    pub material: Rc<dyn Material>,
    pub collider: Option<Collider>,
    pub controller: Option<Box<dyn Controller>>,
}

use crate::scene::context::RenderContext;

impl SceneObject3D {
    pub fn new(renderable: Box<dyn Renderable>, material: Rc<dyn Material>) -> Self {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        Self {
            id,
            name: format!("Object {}", id),
            transform: Transform::default(),
            renderable,
            material,
            collider: None,
            controller: None,
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn with_collider(mut self, collider: Collider) -> Self {
        self.collider = Some(collider);
        self
    }

    pub fn with_controller(mut self, controller: Box<dyn Controller>) -> Self {
        self.controller = Some(controller);
        self
    }

    pub fn update(&mut self, current_time: f32, delta_time: f32) {
        if let Some(ref controller) = self.controller {
            controller.update(&mut self.transform, current_time, delta_time);
        }
    }

    pub fn render(&self, ctx: &RenderContext) {
        self.material.apply();
        let shader = self.material.shader();

        // Matrices
        shader.set_mat4("projection", &ctx.projection.to_cols_array());
        shader.set_mat4("view", &ctx.view.to_cols_array());
        shader.set_mat4("model", &self.transform.to_matrix().to_cols_array());
        shader.set_vec3(
            "u_Scale",
            self.transform.scale.x,
            self.transform.scale.y,
            self.transform.scale.z,
        );

        // Toggles
        shader.set_int("u_UseLighting", if self.material.is_lit() { 1 } else { 0 });
        shader.set_int(
            "u_UseShadows",
            if self.material.receive_shadows() {
                1
            } else {
                0
            },
        );

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

    pub fn destroy(&mut self) {
        println!("Destroying object: {}", self.name);
    }
}

#[allow(dead_code)]
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
