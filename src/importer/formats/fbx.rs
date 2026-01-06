use crate::importer::ImportStrategy;
use russimp::scene::{Scene, PostProcess};

pub struct FbxImporter;

impl ImportStrategy for FbxImporter {
    fn import(&self, path: &str) -> Result<Scene, String> {
        // FBX is complex, often needs triangulation.
        // PresetTargetRealtimeMaxQuality gives a good set of defaults.
        Scene::from_file(
            path, 
            vec![
                PostProcess::Triangulate,
                PostProcess::FlipUVs, 
                PostProcess::LimitBoneWeights, // Good for animation if we support it later
                PostProcess::ValidateDataStructure,
            ]
        ).map_err(|e| e.to_string())
    }
}
