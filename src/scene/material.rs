use std::rc::Rc;
use glam::Vec3;
use crate::shaders::{Shader, Texture};

pub trait Material {
    fn shader(&self) -> &Rc<Shader>;
    fn apply(&self);
    fn is_lit(&self) -> bool { true }
    fn receive_shadows(&self) -> bool { true }
}

pub struct ColoredMaterial {
    pub shader: Rc<Shader>,
    pub color: Vec3,
    pub is_lit: bool,
    pub receive_shadows: bool,
}

impl Default for ColoredMaterial {
    fn default() -> Self {
        panic!("ColoredMaterial requires shader and color");
    }
}

impl Material for ColoredMaterial {
    fn shader(&self) -> &Rc<Shader> {
        &self.shader
    }

    fn apply(&self) {
        self.shader.use_program();
        self.shader.set_vec3("objectColor", self.color.x, self.color.y, self.color.z);
    }
    
    fn is_lit(&self) -> bool { self.is_lit }
    fn receive_shadows(&self) -> bool { self.receive_shadows }
}

pub struct TexturedMaterial {
    pub shader: Rc<Shader>,
    pub texture: Rc<Texture>,
    pub is_lit: bool,
    pub receive_shadows: bool,
}

impl Material for TexturedMaterial {
    fn shader(&self) -> &Rc<Shader> {
        &self.shader
    }

    fn apply(&self) {
        self.shader.use_program();
        self.texture.bind(0);
        self.shader.set_int("u_Texture", 0);
    }
    
    fn is_lit(&self) -> bool { self.is_lit }
    fn receive_shadows(&self) -> bool { self.receive_shadows }
}
