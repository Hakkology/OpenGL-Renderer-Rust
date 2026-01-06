use russimp::scene::Scene;
use crate::texture::Texture;
use image::GenericImageView;

pub mod formats;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AssetType {
    Obj,
    Fbx,
    Blend,
    Png,
    Jpg,
    Unknown,
}

impl AssetType {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "obj" => AssetType::Obj,
            "fbx" => AssetType::Fbx,
            "blend" => AssetType::Blend,
            "png" => AssetType::Png,
            "jpg" | "jpeg" => AssetType::Jpg,
            _ => AssetType::Unknown,
        }
    }
}

pub trait ImportStrategy {
    fn import(&self, path: &str) -> Result<Scene, String>;
}

pub struct AssetImporter;

impl AssetImporter {
    pub fn import_scene(path: &str, asset_type: AssetType) -> Result<Scene, String> {
        let strategy: Box<dyn ImportStrategy> = match asset_type {
            AssetType::Obj => Box::new(formats::obj::ObjImporter),
            AssetType::Fbx => Box::new(formats::fbx::FbxImporter),
            AssetType::Blend => Box::new(formats::blend::BlendImporter),
             _ => return Err(format!("Unsupported scene asset type: {:?} for path: {}", asset_type, path)),
        };
        
        println!("Importing scene {:?} from: {}", asset_type, path);
        strategy.import(path)
    }

    pub fn load_texture(path: &str) -> Result<Texture, String> {
        let img = image::open(path).map_err(|e| format!("Failed to open image {}: {}", path, e))?;
        let (width, height) = img.dimensions();
        let data = img.to_rgba8();
        
        println!("Loaded texture from: {}, size: {}x{}", path, width, height);

        Ok(Texture::new(width, height, &data, gl::RGBA))
    }

    pub fn import_from_extension(path: &str) -> Result<Scene, String> {
        // Legacy wraper for Scenes, keeps API simple for models
        let ext = std::path::Path::new(path)
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("");
            
        let asset_type = AssetType::from_extension(ext);
        Self::import_scene(path, asset_type)
    }
}
