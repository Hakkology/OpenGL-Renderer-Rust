//! Material Factory - Centralized material creation

use super::material::{ColoredMaterial, Material, TexturedMaterial};
use crate::shaders::{Shader, Texture};
use glam::{Vec2, Vec3};
use std::rc::Rc;

/// Factory for creating materials with common presets
pub struct MaterialFactory {
    colored_shader: Rc<Shader>,
    textured_shader: Rc<Shader>,
}

impl MaterialFactory {
    /// Create a new MaterialFactory with the required shaders
    pub fn new(colored_shader: Rc<Shader>, textured_shader: Rc<Shader>) -> Self {
        Self {
            colored_shader,
            textured_shader,
        }
    }

    /// Create a basic colored material with default lighting
    pub fn colored(&self, color: Vec3) -> Rc<dyn Material> {
        Rc::new(ColoredMaterial {
            shader: self.colored_shader.clone(),
            color,
            is_lit: true,
            receive_shadows: true,
        })
    }

    /// Create an unlit colored material (no lighting calculations)
    pub fn colored_unlit(&self, color: Vec3) -> Rc<dyn Material> {
        Rc::new(ColoredMaterial {
            shader: self.colored_shader.clone(),
            color,
            is_lit: false,
            receive_shadows: false,
        })
    }

    /// Create a colored material that doesn't receive shadows
    pub fn colored_no_shadow(&self, color: Vec3) -> Rc<dyn Material> {
        Rc::new(ColoredMaterial {
            shader: self.colored_shader.clone(),
            color,
            is_lit: true,
            receive_shadows: false,
        })
    }

    /// Create a basic textured material with default settings
    pub fn textured(&self, texture: Rc<Texture>) -> Rc<dyn Material> {
        Rc::new(TexturedMaterial {
            shader: self.textured_shader.clone(),
            texture,
            is_lit: true,
            is_repeated: false,
            uv_scale: Vec2::ONE,
            receive_shadows: true,
        })
    }

    /// Create a textured material with tiling/repeat
    pub fn textured_tiled(&self, texture: Rc<Texture>, uv_scale: Vec2) -> Rc<dyn Material> {
        Rc::new(TexturedMaterial {
            shader: self.textured_shader.clone(),
            texture,
            is_lit: true,
            is_repeated: true,
            uv_scale,
            receive_shadows: true,
        })
    }

    /// Create an unlit textured material
    pub fn textured_unlit(&self, texture: Rc<Texture>) -> Rc<dyn Material> {
        Rc::new(TexturedMaterial {
            shader: self.textured_shader.clone(),
            texture,
            is_lit: false,
            is_repeated: false,
            uv_scale: Vec2::ONE,
            receive_shadows: false,
        })
    }

    pub fn red(&self) -> Rc<dyn Material> {
        self.colored(Vec3::new(1.0, 0.0, 0.0))
    }

    pub fn green(&self) -> Rc<dyn Material> {
        self.colored(Vec3::new(0.0, 1.0, 0.0))
    }

    pub fn blue(&self) -> Rc<dyn Material> {
        self.colored(Vec3::new(0.0, 0.0, 1.0))
    }

    pub fn white(&self) -> Rc<dyn Material> {
        self.colored(Vec3::new(1.0, 1.0, 1.0))
    }

    pub fn grey(&self) -> Rc<dyn Material> {
        self.colored(Vec3::new(0.5, 0.5, 0.5))
    }

    pub fn dark_grey(&self) -> Rc<dyn Material> {
        self.colored(Vec3::new(0.3, 0.3, 0.3))
    }

    pub fn light_grey(&self) -> Rc<dyn Material> {
        self.colored(Vec3::new(0.7, 0.7, 0.7))
    }

    pub fn yellow(&self) -> Rc<dyn Material> {
        self.colored(Vec3::new(1.0, 1.0, 0.0))
    }

    pub fn orange(&self) -> Rc<dyn Material> {
        self.colored(Vec3::new(1.0, 0.5, 0.0))
    }

    pub fn purple(&self) -> Rc<dyn Material> {
        self.colored(Vec3::new(0.5, 0.0, 0.5))
    }

    pub fn cyan(&self) -> Rc<dyn Material> {
        self.colored(Vec3::new(0.0, 1.0, 1.0))
    }

    pub fn magenta(&self) -> Rc<dyn Material> {
        self.colored(Vec3::new(1.0, 0.0, 1.0))
    }

    /// Create a grass-green colored material
    pub fn grass_green(&self) -> Rc<dyn Material> {
        self.colored(Vec3::new(0.5, 0.8, 0.2))
    }
}
