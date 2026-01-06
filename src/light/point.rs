use glam::Vec3;
use crate::shaders::Shader;
use super::components::{LightProperties, Attenuation};
use super::Light;

pub struct PointLight {
    pub position: Vec3,
    pub properties: LightProperties,
    pub attenuation: Attenuation,
}

impl PointLight {
    pub fn new(position: Vec3, properties: LightProperties) -> Self {
        Self {
            position,
            properties,
            attenuation: Attenuation::default(),
        }
    }

    pub fn with_attenuation(mut self, attenuation: Attenuation) -> Self {
        self.attenuation = attenuation;
        self
    }

    pub fn simple(position: Vec3, ambient: f32, diffuse: f32, specular: f32, shininess: f32) -> Self {
        Self::new(position, LightProperties::new(ambient, diffuse, specular, shininess))
    }
}

impl Light for PointLight {
    fn apply_to_shader(&self, shader: &Shader, view_pos: Vec3) {
        shader.set_vec3("pointLightPos", self.position.x, self.position.y, self.position.z);
        self.properties.apply_to_shader(shader, "point");
        self.attenuation.apply_to_shader(shader, "point");
        shader.set_vec3("viewPos", view_pos.x, view_pos.y, view_pos.z);
    }
}
