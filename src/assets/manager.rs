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
        let vs_source = std::fs::read_to_string(vert)
            .expect(&format!("Failed to read vertex shader: {}", vert));
        let fs_source = std::fs::read_to_string(frag)
            .expect(&format!("Failed to read fragment shader: {}", frag));

        let fs_source = self.preprocess_shader(&fs_source);

        let shader = Rc::new(
            Shader::from_sources(&vs_source, &fs_source).expect(&format!("Failed to compile shader: {}", name)),
        );
        self.shaders.insert(name.to_string(), shader.clone());
        shader
    }

    fn preprocess_shader(&self, source: &str) -> String {
        let max_lights = crate::config::rendering::MAX_POINT_LIGHTS;

        // 1. Inject MAX_POINT_LIGHTS
        let source = source.replace(
            "#define NR_POINT_LIGHTS 4",
            &format!("#define NR_POINT_LIGHTS {}", max_lights),
        );

        // 2. Unroll Point Shadow Loop if placeholder exists
        if source.contains("{{POINT_SHADOW_LOOP}}") {
            let unrolled_checks = (0..max_lights)
                .map(|i| {
                    format!(
                        "                {} (i == {}) pShadow = calcPointShadow(FragPos, pointLights[{}].position, pointShadowMaps[{}], 15.0);",
                        if i == 0 { "if" } else { "else if" },
                        i, i, i
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");

            let loop_code = format!(
                r#"
        // Generated Unrolled Point Shadow Loop
        for(int i = 0; i < nrPointLights; i++) {{
            float pShadow = 0.0;
            if (u_UseShadows != 0) {{
                vec3 lightToFrag = normalize(FragPos - pointLights[i].position);
                if (dot(norm, -lightToFrag) > 0.0) {{
{}
                }}
            }}
            result += calcPointLight(pointLights[i], norm, viewDir, pShadow);
        }}
"#,
                unrolled_checks
            );

            return source.replace("// {{POINT_SHADOW_LOOP}}", &loop_code);
        }

        source
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
