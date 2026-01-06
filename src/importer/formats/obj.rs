use crate::importer::ImportStrategy;
use russimp::scene::{Scene, PostProcess};

pub struct ObjImporter;

impl ImportStrategy for ObjImporter {
    fn import(&self, path: &str) -> Result<Scene, String> {
        // OBJ loads usually need triangulation and UV flipping for OpenGL
        Scene::from_file(
            path, 
            vec![
                PostProcess::Triangulate,
                PostProcess::FlipUVs,
                PostProcess::JoinIdenticalVertices,
                PostProcess::CalculateTangentSpace
            ]
        ).map_err(|e| e.to_string())
    }
}
