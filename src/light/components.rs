extern crate gl;
use crate::shaders::Shader;
use glam::Vec3;

/// Common lighting components that can be reused across light types
#[derive(Clone, Copy, Debug)]
pub struct LightProperties {
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
    pub color: Vec3,
}

impl Default for LightProperties {
    fn default() -> Self {
        Self {
            ambient: 0.1,
            diffuse: 0.5,
            specular: 1.0,
            shininess: 32.0,
            color: Vec3::ONE,
        }
    }
}

impl LightProperties {
    pub fn new(ambient: f32, diffuse: f32, specular: f32, shininess: f32) -> Self {
        Self {
            ambient,
            diffuse,
            specular,
            shininess,
            color: Vec3::ONE,
        }
    }

    pub fn with_color(mut self, color: Vec3) -> Self {
        self.color = color;
        self
    }

    /// Apply common lighting uniforms to shader
    pub fn apply_to_shader(&self, shader: &Shader, prefix: &str) {
        let ambient_name = if prefix.is_empty() {
            "ambientStrength".to_string()
        } else {
            format!("{}Ambient", prefix)
        };
        let diffuse_name = if prefix.is_empty() {
            "diffuseStrength".to_string()
        } else {
            format!("{}Diffuse", prefix)
        };
        let specular_name = if prefix.is_empty() {
            "specularStrength".to_string()
        } else {
            format!("{}Specular", prefix)
        };
        let shininess_name = if prefix.is_empty() {
            "shininess".to_string()
        } else {
            format!("{}Shininess", prefix)
        };
        let color_name = if prefix.is_empty() {
            "lightColor".to_string()
        } else {
            format!("{}Color", prefix)
        };

        shader.set_float(&ambient_name, self.ambient);
        shader.set_float(&diffuse_name, self.diffuse);
        shader.set_float(&specular_name, self.specular);
        shader.set_float(&shininess_name, self.shininess);
        shader.set_vec3(&color_name, self.color.x, self.color.y, self.color.z);
    }
}

/// Attenuation factors for point and spot lights
#[derive(Clone, Copy, Debug)]
pub struct Attenuation {
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
}

impl Default for Attenuation {
    fn default() -> Self {
        Self {
            constant: 1.0,
            linear: 0.09,
            quadratic: 0.032,
        }
    }
}

impl Attenuation {
    pub fn new(constant: f32, linear: f32, quadratic: f32) -> Self {
        Self {
            constant,
            linear,
            quadratic,
        }
    }

    pub fn apply_to_shader(&self, shader: &Shader, prefix: &str) {
        shader.set_float(&format!("{}Constant", prefix), self.constant);
        shader.set_float(&format!("{}Linear", prefix), self.linear);
        shader.set_float(&format!("{}Quadratic", prefix), self.quadratic);
    }
}

/// Cone shape for spotlights
#[derive(Clone, Copy, Debug)]
pub struct SpotCone {
    pub cut_off: f32,
    pub outer_cut_off: f32,
}

impl Default for SpotCone {
    fn default() -> Self {
        Self {
            cut_off: 12.5f32.to_radians().cos(),
            outer_cut_off: 17.5f32.to_radians().cos(),
        }
    }
}

impl SpotCone {
    pub fn from_degrees(inner: f32, outer: f32) -> Self {
        Self {
            cut_off: inner.to_radians().cos(),
            outer_cut_off: outer.to_radians().cos(),
        }
    }

    pub fn apply_to_shader(&self, shader: &Shader, prefix: &str) {
        shader.set_float(&format!("{}CutOff", prefix), self.cut_off);
        shader.set_float(&format!("{}OuterCutOff", prefix), self.outer_cut_off);
    }
}
