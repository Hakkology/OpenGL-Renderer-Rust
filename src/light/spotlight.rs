use super::components::{Attenuation, LightProperties, SpotCone};
use super::Light;
use crate::shaders::Shader;
use glam::Vec3;

#[derive(Clone, Debug)]
pub struct SpotLight {
    pub position: Vec3,
    pub direction: Vec3,
    pub properties: LightProperties,
    pub cone: SpotCone,
    pub attenuation: Attenuation,
}

impl SpotLight {
    pub fn new(position: Vec3, direction: Vec3, properties: LightProperties) -> Self {
        Self {
            position,
            direction: direction.normalize(),
            properties,
            cone: SpotCone::default(),
            attenuation: Attenuation::default(),
        }
    }

    pub fn with_cone(mut self, cone: SpotCone) -> Self {
        self.cone = cone;
        self
    }

    pub fn with_attenuation(mut self, attenuation: Attenuation) -> Self {
        self.attenuation = attenuation;
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

    pub fn apply_to_shader_indexed(&self, shader: &Shader, index: usize) {
        let prefix = format!("spotLights[{}]", index);
        // Position and Direction directly
        shader.set_vec3(
            &format!("{}.position", prefix),
            self.position.x,
            self.position.y,
            self.position.z,
        );
        shader.set_vec3(
            &format!("{}.direction", prefix),
            self.direction.x,
            self.direction.y,
            self.direction.z,
        );

        // Components - pass prefix with dot for struct access
        let struct_prefix = format!("{}.", prefix);
        self.properties.apply_to_shader(shader, &struct_prefix);
        self.cone.apply_to_shader(shader, &struct_prefix);
        self.attenuation.apply_to_shader(shader, &struct_prefix);
    }
}

impl Light for SpotLight {
    fn apply_to_shader(&self, shader: &Shader, view_pos: Vec3) {
        // This is for single instance usage (not array), adapting to indexed call for 0
        self.apply_to_shader_indexed(shader, 0);
        shader.set_vec3("viewPos", view_pos.x, view_pos.y, view_pos.z);
    }
}
