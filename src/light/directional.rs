use glam::Vec3;
use crate::shaders::Shader;
use super::components::LightProperties;
use super::Light;

pub struct DirectionalLight {
    pub direction: Vec3,
    pub properties: LightProperties,
}

impl DirectionalLight {
    pub fn new(direction: Vec3, properties: LightProperties) -> Self {
        Self {
            direction: direction.normalize(),
            properties,
        }
    }

    pub fn simple(direction: Vec3, ambient: f32, diffuse: f32, specular: f32, shininess: f32) -> Self {
        Self::new(direction, LightProperties::new(ambient, diffuse, specular, shininess))
    }
}

impl Light for DirectionalLight {
    fn apply_to_shader(&self, shader: &Shader, view_pos: Vec3) {
        shader.set_vec3("lightDir", self.direction.x, self.direction.y, self.direction.z);
        self.properties.apply_to_shader(shader, "");
        shader.set_vec3("viewPos", view_pos.x, view_pos.y, view_pos.z);
    }
}
