use crate::importer::ImportStrategy;
use russimp::scene::{Scene, PostProcess};

pub struct BlendImporter;

impl ImportStrategy for BlendImporter {
    fn import(&self, path: &str) -> Result<Scene, String> {
        // Blender files might contain everything.
        Scene::from_file(
            path, 
            vec![
                PostProcess::Triangulate,
                PostProcess::FlipUVs,
                PostProcess::SortByPrimitiveType,
            ]
        ).map_err(|e| e.to_string())
    }
}
