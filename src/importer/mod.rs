use crate::shaders::Texture;
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

    pub fn load_model(path: &str) -> Result<crate::scene::model::Model, String> {
        use russimp::scene::{Scene, PostProcess};
        use crate::scene::model::Mesh;
        
        let scene = Scene::from_file(
            path,
            vec![
                PostProcess::Triangulate,
                PostProcess::FlipUVs,
                PostProcess::JoinIdenticalVertices,
            ],
        ).map_err(|e| format!("Failed to load model {}: {}", path, e))?;

        let mut meshes = Vec::new();

        for mesh in &scene.meshes {
            let mut vertices = Vec::new();
            for i in 0..mesh.vertices.len() {
                // Position
                vertices.push(mesh.vertices[i].x);
                vertices.push(mesh.vertices[i].y);
                vertices.push(mesh.vertices[i].z);

                // TexCoords (take first channel if exists)
                if let Some(coords) = &mesh.texture_coords[0] {
                    vertices.push(coords[i].x);
                    vertices.push(coords[i].y);
                } else {
                    vertices.push(0.0);
                    vertices.push(0.0);
                }

                // Normals
                if mesh.normals.len() > i {
                    vertices.push(mesh.normals[i].x);
                    vertices.push(mesh.normals[i].y);
                    vertices.push(mesh.normals[i].z);
                } else {
                    vertices.push(0.0);
                    vertices.push(0.0);
                    vertices.push(0.0);
                }
            }

            let mut indices = Vec::new();
            for face in &mesh.faces {
                indices.extend_from_slice(&face.0);
            }

            meshes.push(Mesh::new(&vertices, &indices));
        }

        println!("Loaded model from: {}, meshes: {}", path, meshes.len());
        Ok(crate::scene::model::Model::new(meshes))
    }
}
