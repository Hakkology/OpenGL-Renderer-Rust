pub mod components;
pub mod directional;
pub mod point;
pub mod spotlight;

use glam::Vec3;
use crate::shaders::Shader;

// Re-export components
pub use components::{LightProperties, Attenuation, SpotCone};

// Re-export light types
pub use directional::DirectionalLight;
pub use point::PointLight;
pub use spotlight::SpotLight;

/// Common trait for all light types
pub trait Light {
    fn apply_to_shader(&self, shader: &Shader, view_pos: Vec3);
}
