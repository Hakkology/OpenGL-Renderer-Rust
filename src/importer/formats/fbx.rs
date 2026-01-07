use crate::importer::ImportStrategy;
use russimp::scene::{PostProcess, Scene};

pub struct FbxImporter;

impl ImportStrategy for FbxImporter {
    fn import(&self, path: &str) -> Result<Scene, String> {
        // FBX importer
        Scene::from_file(
            path,
            vec![
                PostProcess::Triangulate,
                PostProcess::FlipUVs,
                PostProcess::LimitBoneWeights,
                PostProcess::ValidateDataStructure,
            ],
        )
        .map_err(|e| e.to_string())
    }
}
