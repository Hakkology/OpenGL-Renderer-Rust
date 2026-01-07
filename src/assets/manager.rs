use crate::importer::AssetImporter;
use crate::scene::model::Model;
use crate::shaders::{CubeMap, Shader, Texture};
use std::collections::HashMap;
use std::rc::Rc;

pub struct AssetManager {
    shaders: HashMap<String, Rc<Shader>>,
    textures: HashMap<String, Rc<Texture>>,
    models: HashMap<String, Rc<Model>>,
    cubemaps: HashMap<String, Rc<CubeMap>>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            shaders: HashMap::new(),
            textures: HashMap::new(),
            models: HashMap::new(),
            cubemaps: HashMap::new(),
        }
    }

    pub fn load_shader(&mut self, name: &str, vert: &str, frag: &str) -> Rc<Shader> {
        let shader = Rc::new(
            Shader::from_files(vert, frag).expect(&format!("Failed to load shader: {}", name)),
        );
        self.shaders.insert(name.to_string(), shader.clone());
        shader
    }

    pub fn get_shader(&self, name: &str) -> Option<Rc<Shader>> {
        self.shaders.get(name).cloned()
    }

    pub fn load_texture(&mut self, name: &str, path: &str) -> Rc<Texture> {
        let texture =
            Rc::new(Texture::from_file(path).expect(&format!("Failed to load texture: {}", name)));
        self.textures.insert(name.to_string(), texture.clone());
        texture
    }

    pub fn get_texture(&self, name: &str) -> Option<Rc<Texture>> {
        self.textures.get(name).cloned()
    }

    pub fn load_model(&mut self, name: &str, path: &str) -> Rc<Model> {
        let model = Rc::new(
            AssetImporter::load_model(path).expect(&format!("Failed to load model: {}", name)),
        );
        self.models.insert(name.to_string(), model.clone());
        model
    }

    pub fn get_model(&self, name: &str) -> Option<Rc<Model>> {
        self.models.get(name).cloned()
    }

    pub fn load_cubemap(&mut self, name: &str, path: &str) -> Rc<CubeMap> {
        let cubemap = Rc::new(
            CubeMap::from_cross_file(path).expect(&format!("Failed to load cubemap: {}", name)),
        );
        self.cubemaps.insert(name.to_string(), cubemap.clone());
        cubemap
    }

    pub fn get_cubemap(&self, name: &str) -> Option<Rc<CubeMap>> {
        self.cubemaps.get(name).cloned()
    }
}
