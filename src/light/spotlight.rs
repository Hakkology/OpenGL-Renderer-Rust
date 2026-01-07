use super::components::{LightProperties, SpotCone};
use super::Light;
use crate::shaders::Shader;
use glam::Vec3;

// Not used in the project but can be used later.
#[allow(dead_code)]
pub struct SpotLight {
    pub position: Vec3,
    pub direction: Vec3,
    pub properties: LightProperties,
    pub cone: SpotCone,
}

impl SpotLight {
    pub fn new(position: Vec3, direction: Vec3, properties: LightProperties) -> Self {
        Self {
            position,
            direction: direction.normalize(),
            properties,
            cone: SpotCone::default(),
        }
    }

    pub fn with_cone(mut self, cone: SpotCone) -> Self {
        self.cone = cone;
        self
    }

    pub fn simple(
        position: Vec3,
        direction: Vec3,
        ambient: f32,
        diffuse: f32,
        specular: f32,
        shininess: f32,
    ) -> Self {
        Self::new(
            position,
            direction,
            LightProperties::new(ambient, diffuse, specular, shininess),
        )
    }
}

impl Light for SpotLight {
    fn apply_to_shader(&self, shader: &Shader, view_pos: Vec3) {
        shader.set_vec3(
            "spotLightPos",
            self.position.x,
            self.position.y,
            self.position.z,
        );
        shader.set_vec3(
            "spotLightDir",
            self.direction.x,
            self.direction.y,
            self.direction.z,
        );
        self.properties.apply_to_shader(shader, "spot");
        self.cone.apply_to_shader(shader, "spot");
        shader.set_vec3("viewPos", view_pos.x, view_pos.y, view_pos.z);
    }
}
