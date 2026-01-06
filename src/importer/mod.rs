use crate::texture::Texture;
use image::GenericImageView;

pub mod formats;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AssetType {
    Obj,
    Png,
    Jpg,
    Unknown,
}

impl AssetType {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "obj" => AssetType::Obj,
            "png" => AssetType::Png,
            "jpg" | "jpeg" => AssetType::Jpg,
            _ => AssetType::Unknown,
        }
    }
}

pub struct AssetImporter;

impl AssetImporter {
    pub fn load_texture(path: &str) -> Result<Texture, String> {
        let img = image::open(path).map_err(|e| format!("Failed to open image {}: {}", path, e))?;
        let (width, height) = img.dimensions();
        let data = img.to_rgba8();
        
        println!("Loaded texture from: {}, size: {}x{}", path, width, height);

        Ok(Texture::new(width, height, &data, gl::RGBA))
    }
}
